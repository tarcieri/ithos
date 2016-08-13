use std::collections::HashMap;
use std::io;
use std::string::ToString;

use buffoon::{OutputStream, Serialize};

use adapter::Adapter;
use block::Block;
use entry::{self, Entry, TypeId};
use error::Result;
use metadata::Metadata;
use objectclass::ObjectClass;
use objecthash::{self, ObjectHash, ObjectHasher};
use path::Path;
use proto::ToProto;

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

impl ObjectHash for OpType {
    #[inline]
    fn objecthash<H: ObjectHasher>(&self, hasher: &mut H) {
        self.to_string().objecthash(hasher);
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
        let proto = try!(self.objectclass.to_proto());
        let entry = Entry {
            type_id: TypeId::from_objectclass(&self.objectclass),
            data: &proto,
        };

        // NOTE: The underlying adapter must handle Error::EntryAlreadyExists
        try!(adapter.add_entry(txn, entry_id, parent_id, &name, &metadata, &entry));
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
    #[inline]
    fn objecthash<H: ObjectHasher>(&self, hasher: &mut H) {
        objecthash_struct!(
            hasher,
            "optype" => self.optype,
            "path" => self.path,
            "objectclass" => self.objectclass
        )
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
