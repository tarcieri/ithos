use std::io;
use std::string::ToString;

use buffoon::{OutputStream, Serialize};
use ring::digest;

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
    pub data: Vec<u8>,
}

impl ToString for OpType {
    fn to_string(&self) -> String {
        match *self {
            OpType::Add => "add".to_string(),
        }
    }
}

impl Op {
    pub fn new(optype: OpType, path: Path, objectclass: ObjectClass, data: &[u8]) -> Op {
        Op {
            optype: optype,
            path: path,
            objectclass: objectclass,
            data: Vec::from(data),
        }
    }
}

impl Serialize for Op {
    fn serialize<O: OutputStream>(&self, out: &mut O) -> io::Result<()> {
        try!(out.write(1, &(self.optype as u32 + 1)));
        try!(out.write(2, &self.path.to_string()));
        try!(out.write(3, &(self.objectclass as u32 + 1)));
        try!(out.write(4, &self.data));
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

        ctx.update("data".objecthash().as_ref());
        ctx.update(self.data.objecthash().as_ref());

        ctx.finish()
    }
}
