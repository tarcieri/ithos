use std::io;

use buffoon::{self, OutputStream, Serialize};

use block;
use error::{Error, Result};
use objectclass::ObjectClass;

pub struct Metadata {
    pub objectclass: ObjectClass,
    pub created_id: block::Id,
    pub updated_id: block::Id,
    pub created_at: u64,
    pub updated_at: u64,
    pub version: u64,
}

impl Metadata {
    pub fn new(objectclass: ObjectClass, id: block::Id, timestamp: u64) -> Metadata {
        Metadata {
            objectclass: objectclass,
            created_id: id,
            updated_id: id,
            created_at: timestamp,
            updated_at: timestamp,
            version: 0,
        }
    }

    pub fn to_proto(&self) -> Result<Vec<u8>> {
        buffoon::serialize(&self).map_err(|_| Error::Serialize)
    }
}

impl Serialize for Metadata {
    fn serialize<O: OutputStream>(&self, out: &mut O) -> io::Result<()> {
        try!(out.write(1, &(self.objectclass as u32 + 1)));
        try!(out.write(2, self.created_id.as_ref()));
        try!(out.write(3, self.updated_id.as_ref()));
        try!(out.write(4, &self.created_at));
        try!(out.write(5, &self.updated_at));
        try!(out.write(6, &self.version));
        Ok(())
    }
}
