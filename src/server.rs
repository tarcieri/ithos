use std::{result, mem, str};

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Error {
    DbCreateError,
    DbOpenError,
    DbWriteError,
    DbCorruptError,
    ParseError,
    TransactionError,
    PathError,
    NotFoundError,
    DuplicateEntryError,
}

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct Id(u64);

#[derive(Debug, Eq, PartialEq)]
pub struct Node<'a> {
    pub id: Id,
    pub parent_id: Id,
    pub name: &'a str,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Entry<'a> {
    pub node: Node<'a>,
    pub objectclass: &'a str,
}

// Ids are 64-bit integers in host-native byte order
// LMDB has special optimizations for host-native integers as keys
impl Id {
    pub fn root() -> Id {
        Id(0)
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Id> {
        if bytes.len() != 8 {
            return Err(Error::ParseError);
        }

        let mut id = [0u8; 8];
        id.copy_from_slice(&bytes[0..8]);

        Ok(Id(unsafe { mem::transmute(id) }))
    }

    pub fn as_bytes(self) -> [u8; 8] {
        let Id(id) = self;
        unsafe { mem::transmute(id) }
    }

    pub fn next(self) -> Id {
        let Id(id) = self;
        Id(id + 1)
    }
}

impl<'a> Node<'a> {
    pub fn root() -> Node<'a> {
        Node {
            id: Id::root(),
            parent_id: Id::root(),
            name: "/",
        }
    }

    pub fn from_parent_id_and_bytes(parent_id: Id, bytes: &[u8]) -> Result<Node> {
        if bytes.len() < 8 {
            return Err(Error::DbCorruptError);
        }

        let id = try!(Id::from_bytes(&bytes[0..8]));
        let name = try!(str::from_utf8(&bytes[8..]).map_err(|_err| Error::DbCorruptError));

        Ok(Node {
            id: id,
            parent_id: parent_id,
            name: name,
        })
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(8 + self.name.len());
        bytes.extend_from_slice(&self.id.as_bytes());
        bytes.extend_from_slice(self.name.as_bytes());
        bytes
    }
}
