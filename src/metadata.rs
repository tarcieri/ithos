use std::io;

use buffoon::{Serialize, Deserialize, OutputStream, InputStream};

use block;
use proto::{FromProto, ToProto};
use timestamp::Timestamp;

pub struct Metadata {
    pub created_id: block::Id,
    pub updated_id: block::Id,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
    pub version: u64,
}

impl Metadata {
    pub fn new(id: block::Id, timestamp: Timestamp) -> Metadata {
        Metadata {
            created_id: id,
            updated_id: id,
            created_at: timestamp,
            updated_at: timestamp,
            version: 0,
        }
    }
}

impl FromProto for Metadata {}
impl ToProto for Metadata {}

impl Serialize for Metadata {
    fn serialize<O: OutputStream>(&self, out: &mut O) -> io::Result<()> {
        try!(out.write(1, self.created_id.as_ref()));
        try!(out.write(2, self.updated_id.as_ref()));
        try!(out.write(3, &self.created_at));
        try!(out.write(4, &self.updated_at));
        try!(out.write(5, &self.version));
        Ok(())
    }
}

impl Deserialize for Metadata {
    fn deserialize<R: io::Read>(i: &mut InputStream<R>) -> io::Result<Metadata> {
        let mut created_id_bytes: Option<Vec<u8>> = None;
        let mut updated_id_bytes: Option<Vec<u8>> = None;
        let mut created_at = None;
        let mut updated_at = None;
        let mut version = None;

        while let Some(f) = try!(i.read_field()) {
            match f.tag() {
                1 => created_id_bytes = Some(try!(f.read())),
                2 => updated_id_bytes = Some(try!(f.read())),
                3 => created_at = Some(try!(f.read())),
                4 => updated_at = Some(try!(f.read())),
                5 => version = Some(try!(f.read())),
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
            created_id: created_id,
            updated_id: updated_id,
            created_at: required!(created_at, "Metadata::created_at"),
            updated_at: required!(updated_at, "Metadata::updated_at"),
            version: required!(version, "Metadata::version"),
        })
    }
}
