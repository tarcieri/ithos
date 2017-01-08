use adapter::Adapter;
use block;
use buffoon::{OutputStream, Serialize};
use entry::{self, Entry};
use error::{Error, Result};
use metadata::Metadata;
use object::{Class, Object};
use objecthash::{self, ObjectHash, ObjectHasher};
use path::{Path, PathBuf};
use serde_json::builder::ObjectBuilder;
use std::collections::HashMap;
use std::io;
use std::string::ToString;
use timestamp::Timestamp;

#[cfg(test)]
extern crate tempdir;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Type {
    Add,
}

impl ObjectHash for Type {
    #[inline]
    fn objecthash<H: ObjectHasher>(&self, hasher: &mut H) {
        self.to_string().objecthash(hasher);
    }
}

impl Serialize for Type {
    fn serialize<O: OutputStream>(&self, _: &mut O) -> io::Result<()> {
        unimplemented!();
    }

    fn serialize_nested<O: OutputStream>(&self, field: u32, out: &mut O) -> io::Result<()> {
        out.write_varint(field, *self as u32)
    }
}

impl ToString for Type {
    fn to_string(&self) -> String {
        match *self {
            Type::Add => "ADD".to_string(),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Op {
    pub optype: Type,
    pub path: PathBuf,
    pub object: Object,
}

impl Op {
    pub fn new(optype: Type, path: PathBuf, object: Object) -> Op {
        Op {
            optype: optype,
            path: path,
            object: object,
        }
    }

    pub fn apply<'a, A>(&self,
                        adapter: &A,
                        txn: &mut A::W,
                        state: &mut State,
                        block_id: &block::Id,
                        timestamp: Timestamp)
                        -> Result<()>
        where A: Adapter<'a>
    {
        match self.optype {
            Type::Add => self.add(adapter, txn, state, block_id, timestamp),
        }
    }

    pub fn build_json(&self, builder: ObjectBuilder) -> ObjectBuilder {
        builder.insert("optype", self.optype.to_string())
            .insert("path", self.path.as_path().to_string())
            .insert_object("object", |b| self.object.build_json(b))
    }

    fn add<'a, A>(&self,
                  adapter: &A,
                  txn: &mut A::W,
                  state: &mut State,
                  block_id: &block::Id,
                  timestamp: Timestamp)
                  -> Result<()>
        where A: Adapter<'a>
    {
        let parent_path = self.path.as_path().parent();

        let (parent_id, entry_id) = match parent_path {
            Some(path) => {
                let parent_entry = try!(state.get_entry(adapter, txn, path));

                if !parent_entry.class.allows_child(&self.object) {
                    return Err(Error::nesting_invalid(None));
                }

                (parent_entry.id, state.get_next_entry_id())
            }
            None => {
                if self.object.class() != Class::Root {
                    return Err(Error::nesting_invalid(None));
                }

                (entry::Id::root(), entry::Id::root())
            }
        };

        let name = try!(self.path.as_path().entry_name().ok_or(Error::path_invalid(None)));
        let metadata = Metadata::new(*block_id, timestamp);
        let proto = try!(self.object.to_proto());

        let entry = Entry {
            id: entry_id,
            class: self.object.class(),
            data: &proto,
        };

        // NOTE: The underlying adapter must handle Error::EntryAlreadyExists
        try!(adapter.add_entry(txn, &entry, &name, parent_id, &metadata));

        let new_entry = StateEntry {
            id: entry_id,
            class: self.object.class(),
        };
        state.new_entries.insert(self.path.clone(), new_entry);

        Ok(())
    }
}

impl Serialize for Op {
    fn serialize<O: OutputStream>(&self, out: &mut O) -> io::Result<()> {
        try!(out.write(1, &self.optype));
        try!(out.write(2, &self.path.as_path().to_string()));
        try!(out.write(3, &self.object));

        Ok(())
    }
}

impl ObjectHash for Op {
    #[inline]
    fn objecthash<H: ObjectHasher>(&self, hasher: &mut H) {
        objecthash_struct!(
            hasher,
            "optype" => self.optype,
            "path" => self.path,
            "object" => self.object
        )
    }
}

pub struct State {
    next_entry_id: entry::Id,
    new_entries: HashMap<PathBuf, StateEntry>,
}

impl State {
    pub fn new(next_entry_id: entry::Id) -> State {
        State {
            next_entry_id: next_entry_id,
            new_entries: HashMap::new(),
        }
    }

    fn get_next_entry_id(&mut self) -> entry::Id {
        let id = self.next_entry_id;
        self.next_entry_id = id.next();
        id
    }

    fn get_entry<'a, A>(&self, adapter: &A, txn: &mut A::W, path: &Path) -> Result<StateEntry>
        where A: Adapter<'a>
    {
        match self.new_entries.get(path) {
            Some(parent_entry) => Ok(*parent_entry),
            _ => {
                let id = try!(adapter.find_direntry(txn, path)).id;
                let class = try!(adapter.find_entry(txn, &id)).class;
                Ok(StateEntry {
                    id: id,
                    class: class,
                })
            }
        }
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
struct StateEntry {
    id: entry::Id,
    class: Class,
}

#[cfg(test)]
pub mod tests {

    use adapter::Adapter;
    use adapter::lmdb::LmdbAdapter;
    use algorithm::DigestAlgorithm;
    use block;
    use error::Error;
    use object::Object;
    use object::domain::DomainEntry;
    use object::root::RootEntry;
    use path::PathBuf;

    use super::{Op, State, Type};
    use super::tempdir::TempDir;
    use timestamp::Timestamp;

    fn test_adapter() -> LmdbAdapter {
        let dir = TempDir::new("ithos-test").unwrap();
        LmdbAdapter::create_database(dir.path()).unwrap()
    }

    fn example_timestamp() -> Timestamp {
        Timestamp::at(1_231_006_505)
    }

    #[test]
    fn nesting_constraint_violation() {
        let adapter = test_adapter();

        // Test nesting constraints on root entry
        {
            let mut txn = adapter.rw_transaction().unwrap();

            let example_block_id = block::Id::zero();

            let op = Op::new(Type::Add,
                             PathBuf::from("/".to_string()),
                             Object::Domain(DomainEntry::new(None)));

            let mut state = State::new(adapter.next_free_entry_id(&txn).unwrap());

            let result = op.apply(&adapter,
                                  &mut txn,
                                  &mut state,
                                  &example_block_id,
                                  example_timestamp());

            assert_eq!(result, Err(Error::nesting_invalid(None)));
        }

        // Test nesting constraints on a non-root entry
        {
            let mut txn = adapter.rw_transaction().unwrap();

            let example_block_id = block::Id::zero();

            let op1 = Op::new(Type::Add,
                              PathBuf::from("/".to_string()),
                              Object::Root(RootEntry::new(DigestAlgorithm::Sha256)));

            let op2 = Op::new(Type::Add,
                              PathBuf::from("/derp".to_string()),
                              Object::Root(RootEntry::new(DigestAlgorithm::Sha256)));

            let mut state = State::new(adapter.next_free_entry_id(&txn).unwrap());

            let result1 = op1.apply(&adapter,
                                    &mut txn,
                                    &mut state,
                                    &example_block_id,
                                    example_timestamp());

            assert!(result1.is_ok());

            let result2 = op2.apply(&adapter,
                                    &mut txn,
                                    &mut state,
                                    &example_block_id,
                                    example_timestamp());

            assert_eq!(result2, Err(Error::nesting_invalid(None)));
        }
    }
}
