use std::collections::HashMap;
use std::io;
use std::string::ToString;

use buffoon::{OutputStream, Serialize};
use serde_json::builder::ObjectBuilder;

use adapter::Adapter;
use block::Block;
use entry::{self, Entry};
use error::{Error, Result};
use metadata::Metadata;
use object::{Class, Object};
use objecthash::{self, ObjectHash, ObjectHasher};
use path::{Path, PathBuf};
use proto::ToProto;

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
        out.write_varint(field, *self as u32 + 1)
    }
}

impl ToString for Type {
    fn to_string(&self) -> String {
        match *self {
            Type::Add => "ADD".to_string(),
        }
    }
}

pub struct Op {
    pub optype: Type,
    pub path: PathBuf,
    pub object: Object,
}

impl Op {
    pub fn new(optype: Type, path: PathBuf, objectclass: Object) -> Op {
        Op {
            optype: optype,
            path: path,
            object: objectclass,
        }
    }

    pub fn apply<'a, 'b, A: Adapter<'a>>(&'b self,
                                         adapter: &A,
                                         txn: &mut A::W,
                                         state: &mut State<'b>,
                                         block: &Block)
                                         -> Result<()> {
        match self.optype {
            Type::Add => self.add(adapter, txn, state, block),
        }
    }

    pub fn build_json(&self, builder: ObjectBuilder) -> ObjectBuilder {
        builder.insert("optype", self.optype.to_string())
            .insert("path", self.path.as_path().to_string())
            .insert_object("objectclass", |b| self.object.build_json(b))
    }

    fn add<'a, 'b, A: Adapter<'a>>(&'b self,
                                   adapter: &A,
                                   txn: &mut A::W,
                                   state: &mut State<'b>,
                                   block: &Block)
                                   -> Result<()> {
        let entry_id = state.get_next_entry_id();

        let parent_path = self.path.as_path().parent();
        let parent_id = match parent_path {
            Some(parent) => {
                match state.new_entries.get(parent) {
                    Some(&(id, parent_class)) => {
                        if !parent_class.allows_child(&self.object) {
                            return Err(Error::ObjectNestingError);
                        }
                        id
                    }
                    _ => {
                        let id = try!(adapter.find_direntry(txn, parent)).id;
                        let parent_entry = try!(adapter.find_entry(txn, &id));

                        if !parent_entry.class.allows_child(&self.object) {
                            return Err(Error::ObjectNestingError);
                        }

                        id
                    }
                }
            }
            None => entry::Id::root(),
        };

        let name = try!(self.path.as_path().entry_name().ok_or(Error::PathInvalid));
        let metadata = Metadata::new(block.id, block.timestamp);
        let proto = try!(self.object.to_proto());
        let entry = Entry {
            id: entry_id,
            class: self.object.class(),
            data: &proto,
        };

        // NOTE: The underlying adapter must handle Error::EntryAlreadyExists
        try!(adapter.add_entry(txn, &entry, &name, parent_id, &metadata));
        state.new_entries.insert(self.path.as_path(), (entry_id, self.object.class()));

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

pub struct State<'a> {
    next_entry_id: entry::Id,
    new_entries: HashMap<&'a Path, (entry::Id, Class)>,
}

impl<'a> State<'a> {
    pub fn new(next_entry_id: entry::Id) -> State<'a> {
        State {
            next_entry_id: next_entry_id,
            new_entries: HashMap::new(),
        }
    }

    pub fn get_next_entry_id(&mut self) -> entry::Id {
        let id = self.next_entry_id;
        self.next_entry_id = id.next();
        id
    }
}
