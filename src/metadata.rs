use std::io;

use buffoon::{self, Serialize, Deserialize, OutputStream, InputStream};

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

    pub fn from_proto(bytes: &[u8]) -> Result<Metadata> {
        buffoon::deserialize(bytes).map_err(|_| Error::Parse)
    }

    pub fn to_proto(&self) -> Result<Vec<u8>> {
        buffoon::serialize(&self).map_err(|_| Error::Serialize)
    }
}

impl Serialize for Metadata {
    fn serialize<O: OutputStream>(&self, out: &mut O) -> io::Result<()> {
        try!(out.write(1, &self.objectclass));
        try!(out.write(2, self.created_id.as_ref()));
        try!(out.write(3, self.updated_id.as_ref()));
        try!(out.write(4, &self.created_at));
        try!(out.write(5, &self.updated_at));
        try!(out.write(6, &self.version));
        Ok(())
    }
}

impl Deserialize for Metadata {
    fn deserialize<R: io::Read>(i: &mut InputStream<R>) -> io::Result<Metadata> {
        let mut objectclass = None;
        let mut created_id_bytes: Option<Vec<u8>> = None;
        let mut updated_id_bytes: Option<Vec<u8>> = None;
        let mut created_at = None;
        let mut updated_at = None;
        let mut version = None;

        while let Some(f) = try!(i.read_field()) {
            match f.tag() {
                1 => objectclass = Some(try!(f.read())),
                2 => created_id_bytes = Some(try!(f.read())),
                3 => updated_id_bytes = Some(try!(f.read())),
                4 => created_at = Some(try!(f.read())),
                5 => updated_at = Some(try!(f.read())),
                6 => version = Some(try!(f.read())),
                _ => try!(f.skip()),
            }
        }

        let created_id = try!(block::Id::from_bytes(&required!(created_id_bytes,
                                                               "Metadata::created_id"))
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "error parsing created_id")));

        let updated_id = try!(block::Id::from_bytes(&required!(updated_id_bytes,
                                                               "Metadata::updated_id"))
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "error parsing updated_id")));

        Ok(Metadata {
            objectclass: required!(objectclass, "Metadata::objectclass"),
            created_id: created_id,
            updated_id: updated_id,
            created_at: required!(created_at, "Metadata::created_at"),
            updated_at: required!(updated_at, "Metadata::updated_at"),
            version: required!(version, "Metadata::version"),
        })
    }
}
