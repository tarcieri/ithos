use std::collections::HashMap;
use std::io;
use std::string::ToString;

use buffoon::{OutputStream, Serialize};
use ring::digest;

use adapter::Adapter;
use block::Block;
use entry;
use error::Result;
use metadata::Metadata;
use objectclass::ObjectClass;
use objecthash::{ObjectHash, DIGEST_ALG};
use path::Path;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum OpType {
    Add,
}

pub struct Op {
    pub optype: OpType,
    pub path: Path,
    pub objectclass: ObjectClass,
}

pub struct State {
    pub next_entry_id: entry::Id,
    pub new_entries: HashMap<Path, entry::Id>,
}

impl ToString for OpType {
    fn to_string(&self) -> String {
        match *self {
            OpType::Add => "ADD".to_string(),
        }
    }
}

impl Serialize for OpType {
    fn serialize<O: OutputStream>(&self, _: &mut O) -> io::Result<()> {
        unimplemented!();
    }

    fn serialize_nested<O: OutputStream>(&self, field: u32, out: &mut O) -> io::Result<()> {
        out.write_varint(field, *self as u32 + 1)
    }
}

impl Op {
    pub fn new(optype: OpType, path: Path, objectclass: ObjectClass) -> Op {
        Op {
            optype: optype,
            path: path,
            objectclass: objectclass,
        }
    }

    pub fn apply<'a, A: Adapter<'a>>(&self,
                                     adapter: &A,
                                     txn: &mut A::W,
                                     state: &mut State,
                                     block: &Block)
                                     -> Result<()> {
        match self.optype {
            OpType::Add => self.add(adapter, txn, state, block),
        }
    }

    fn add<'a, A: Adapter<'a>>(&self,
                               adapter: &A,
                               txn: &mut A::W,
                               state: &mut State,
                               block: &Block)
                               -> Result<()> {
        let entry_id = state.get_entry_id();

        let parent_id = if self.path.is_root() {
            entry::Id::root()
        } else {
            match state.new_entries.get(&self.path.parent()) {
                Some(&id) => id,
                _ => try!(adapter.find_direntry(txn, &self.path.parent())).id,
            }
        };

        let name = self.path.name();
        let metadata = Metadata::new(block.id, block.timestamp);

        // NOTE: The underlying adapter must handle Error::EntryAlreadyExists
        try!(adapter.add_entry(txn,
                               entry_id,
                               parent_id,
                               &name,
                               &metadata,
                               &self.objectclass));
        state.new_entries.insert(self.path.clone(), entry_id);

        Ok(())
    }
}

impl Serialize for Op {
    fn serialize<O: OutputStream>(&self, out: &mut O) -> io::Result<()> {
        try!(out.write(1, &self.optype));
        try!(out.write(2, &self.path.to_string()));
        try!(out.write(3, &self.objectclass));

        Ok(())
    }
}

impl ObjectHash for Op {
    fn objecthash(&self) -> digest::Digest {
        let mut ctx = digest::Context::new(&DIGEST_ALG);

        // objecthash qualifier for dictionaries
        ctx.update(b"d");

        // OpType::Add is the only op we support right now
        assert!(self.optype == OpType::Add);

        ctx.update("optype".objecthash().as_ref());
        ctx.update(self.optype.to_string().objecthash().as_ref());

        ctx.update("path".objecthash().as_ref());
        ctx.update(self.path.to_string().objecthash().as_ref());

        ctx.update("objectclass".objecthash().as_ref());
        ctx.update(self.objectclass.objecthash().as_ref());

        ctx.finish()
    }
}

impl State {
    pub fn new(next_entry_id: entry::Id) -> State {
        State {
            next_entry_id: next_entry_id,
            new_entries: HashMap::new(),
        }
    }

    pub fn get_entry_id(&mut self) -> entry::Id {
        let id = self.next_entry_id;
        self.next_entry_id = id.next();
        id
    }
}
