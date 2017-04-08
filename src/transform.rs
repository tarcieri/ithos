//! transform.rs: Apply the operations in a block to the current database state
//!
//! This module should ideally be the one true path all adapter writes happen through,
//! as all writes to the database are mediated by the log.
//!
//! BIG SECURITY WARNING: This module does not perform cryptographic authorization checks.
//! TODO: It should!
//!

use adapter::{Adapter, Transaction};
use block::Block;
use entry::{Class, Entry, SerializedEntry};
use errors::*;
use id::{BlockId, EntryId};
use metadata::Metadata;
use op::{self, Op};
use path::{Path, PathBuf};
use std::collections::HashMap;
use timestamp::Timestamp;

#[cfg(test)]
extern crate tempdir;

/// An uncommitted entry being processed during a transactional transform
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
struct TransformEntry {
    id: EntryId,
    class: Class,
}

/// Transactionally apply the operations contained within a block to the
/// database, aborting if there are any consistency errors
pub struct Transform<'a, A: Adapter<'a> + 'a> {
    adapter: &'a A,
    txn: A::W,
    next_entry_id: EntryId,
    new_entries: HashMap<PathBuf, TransformEntry>,
}

impl<'a, A: Adapter<'a> + 'a> Transform<'a, A> {
    /// Create a new transform for the current adapter
    pub fn new(adapter: &'a A) -> Result<Transform<'a, A>> {
        let txn = adapter.rw_transaction()?;
        let next_entry_id = adapter.next_free_entry_id(&txn)?;

        Ok(Transform {
            adapter: adapter,
            txn: txn,
            next_entry_id: next_entry_id,
            new_entries: HashMap::new(),
        })
    }

    /// Obtain the current block ID (i.e. prior to the transaction being committed)
    pub fn block_id(&mut self) -> Result<BlockId> {
        self.adapter.current_block_id(&self.txn)
    }

    /// Apply the operations in the given block to the database
    pub fn apply(&mut self, block: &Block) -> Result<()> {
        // NOTE: This only stores the block in the database. It does not process it
        self.adapter.add_block(&mut self.txn, block)?;

        let block_id = BlockId::of(block);
        let ops = &block.get_body().get_ops();

        // Process the operations in the block and apply them to the database
        for op in ops.iter() {
            match op.get_optype() {
                op::Type::ADD => {
                    self.add(op,
                             &block_id,
                             Timestamp::at(block.get_body().get_timestamp()))?
                }
            };
        }

        Ok(())

    }

    /// Commit the transaction in which the ops contained in a block have been applied
    pub fn commit(self) -> Result<()> {
        self.txn.commit()
    }

    /// Add a new entry to the directory tree
    fn add(&mut self, op: &Op, block_id: &BlockId, timestamp: Timestamp) -> Result<()> {
        let child_path = Path::new(op.get_path())
            .ok_or_else(|| ErrorKind::PathInvalid(format!("bad path: {}", op.get_path())))?;

        let parent_path = child_path.parent();
        let child_class =
            Class::from_object(op.get_object()).ok_or_else(|| {
                    ErrorKind::TypeInvalid(format!("bad object type: {:?}", op.get_object()))
                })?;

        let (parent_id, entry_id) = match parent_path {
            Some(path) => {
                let parent_entry = self.get_entry(path)?;

                if !parent_entry.class.allows_child(&child_class) {
                    let msg = format!("{:?} does not allow {:?} as child",
                                      parent_entry.class,
                                      child_class);
                    return Err(ErrorKind::StructureInvalid(msg).into());
                }

                let next_id = self.next_entry_id;
                self.next_entry_id = next_id.next();

                (parent_entry.id, next_id)
            }
            None => {
                if child_class != Class::Root {
                    let msg = format!("{:?} not allowed as root entry", child_class);
                    return Err(ErrorKind::StructureInvalid(msg).into());
                }

                (EntryId::root(), EntryId::root())
            }
        };

        let mut metadata = Metadata::new();

        metadata.set_created_id(Vec::from(block_id.as_ref()));
        metadata.set_updated_id(Vec::from(block_id.as_ref()));
        metadata.set_created_at(timestamp.to_int());
        metadata.set_updated_at(timestamp.to_int());

        let entry =
            Entry::from_object(&mut op.get_object().clone()).ok_or_else(|| {
                    ErrorKind::SerializationFailure("unsupported object type".to_string())
                })?;

        let entry = SerializedEntry {
            id: entry_id,
            class: child_class,
            data: &entry.serialize()?,
        };

        let entry_name =
            child_path.entry_name()
                .ok_or_else(|| {
                    ErrorKind::PathInvalid(format!("missing entry name: {:?}", child_path))
                })?;

        // NOTE: The underlying adapter must handle Error::EntryAlreadyExists
        self.adapter.add_entry(&mut self.txn, &entry, entry_name, parent_id, &metadata)?;

        let new_entry = TransformEntry {
            id: entry_id,
            class: child_class,
        };

        self.new_entries.insert(child_path.to_owned(), new_entry);

        Ok(())
    }

    /// Get an entry, either from the buffer of uncommitted entries, or failing that, by reading
    /// from the current view of the directory tree
    fn get_entry(&self, path: &Path) -> Result<TransformEntry> {
        if let Some(parent_entry) = self.new_entries.get(path) {
            Ok(*parent_entry)
        } else {
            let id = try!(self.adapter.find_direntry(&self.txn, path)).id;
            let class = try!(self.adapter.find_entry(&self.txn, &id)).class;

            Ok(TransformEntry {
                id: id,
                class: class,
            })
        }
    }
}

#[cfg(test)]
pub mod tests {
    use adapter::Adapter;
    use adapter::lmdb::LmdbAdapter;
    use alg::DigestAlg;
    use block::{Block, Body};
    use crypto::signing::KeyPair;
    use errors::*;
    use id::BlockId;
    use object::Object;
    use object::domain::Domain;
    use object::root::Root;
    use op::{self, Op};
    use protobuf::RepeatedField;
    use ring::rand;
    use timestamp::Timestamp;
    use transform::Transform;
    use transform::tempdir::TempDir;

    const COMMENT: &'static str = "The tree of a thousand users begins with a single block";

    fn test_adapter() -> LmdbAdapter {
        let dir = TempDir::new("ithos-test").unwrap();
        LmdbAdapter::create_database(dir.path()).unwrap()
    }

    fn example_block(parent_id: BlockId, ops: Vec<Op>) -> Block {
        let mut body = Body::new();
        body.set_parent_id(Vec::from(parent_id.as_ref()));
        body.set_timestamp(Timestamp::at(1_231_006_505).to_int());
        body.set_ops(RepeatedField::from_vec(ops));
        body.set_comment(COMMENT.to_owned());

        let rng = rand::SystemRandom::new();
        KeyPair::generate(&rng).sign_block(body)
    }

    #[test]
    fn root_nesting_constraint() {
        let adapter = test_adapter();
        let mut transform = Transform::new(&adapter).unwrap();

        let mut domain_object = Object::new();
        domain_object.set_domain(Domain::new());

        let mut op = Op::new();
        op.set_optype(op::Type::ADD);
        op.set_path("/".to_string());
        op.set_object(domain_object);

        let block = example_block(BlockId::zero(), vec![op]);

        match *transform.apply(&block).expect_err("expected a structural error").kind() {
            ErrorKind::StructureInvalid(_) => (),
            ref other => panic!("unexpected error: {:?}", other),
        }
    }

    #[test]
    fn non_root_nesting_constraint() {
        let adapter = test_adapter();
        let mut transform = Transform::new(&adapter).unwrap();

        let mut root1 = Root::new();
        root1.set_digest_alg(DigestAlg::SHA256);

        let root2 = root1.clone();

        let mut root1_object = Object::new();
        root1_object.set_root(root1);

        let mut op1 = Op::new();
        op1.set_optype(op::Type::ADD);
        op1.set_path("/".to_string());
        op1.set_object(root1_object);

        let block1 = example_block(BlockId::zero(), vec![op1]);
        assert!(transform.apply(&block1).is_ok());

        let mut root2_object = Object::new();
        root2_object.set_root(root2);

        let mut op2 = Op::new();
        op2.set_optype(op::Type::ADD);
        op2.set_path("/derp".to_string());
        op2.set_object(root2_object);

        let block2 = example_block(BlockId::of(&block1), vec![op2]);
        match *transform.apply(&block2).expect_err("expected a structural error").kind() {
            ErrorKind::StructureInvalid(_) => (),
            ref other => panic!("unexpected error kind: {:?}", other),
        }
    }
}
