

use entry::Id;
use error::{Error, Result};
use std::str;

#[derive(Debug, Eq, PartialEq)]
pub struct DirEntry<'a> {
    pub id: Id,
    pub parent_id: Id,
    pub name: &'a str,
}

impl<'a> DirEntry<'a> {
    pub fn root() -> DirEntry<'a> {
        DirEntry {
            id: Id::root(),
            parent_id: Id::root(),
            name: "/",
        }
    }

    pub fn new(parent_id: Id, bytes: &[u8]) -> Result<DirEntry> {
        if bytes.len() < 8 {
            return Err(Error::Parse);
        }

        let id = try!(Id::from_bytes(&bytes[0..8]));
        let name = try!(str::from_utf8(&bytes[8..]).map_err(|_| Error::Parse));

        Ok(DirEntry {
            id: id,
            parent_id: parent_id,
            name: name,
        })
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(8 + self.name.len());
        bytes.extend_from_slice(self.id.as_ref());
        bytes.extend_from_slice(self.name.as_bytes());
        bytes
    }
}
