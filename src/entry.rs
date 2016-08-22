use std::mem;

use error::{Error, Result};
use objectclass::ObjectClass;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct Id(u64);

// Ids are 64-bit integers in host-native byte order
// LMDB has special optimizations for host-native integers as keys
impl Id {
    pub fn root() -> Id {
        Id(0)
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Id> {
        if bytes.len() != 8 {
            return Err(Error::Parse);
        }

        let mut id = [0u8; 8];
        id.copy_from_slice(&bytes[0..8]);

        Ok(Id(unsafe { mem::transmute(id) }))
    }

    pub fn next(self) -> Id {
        Id(self.0 + 1)
    }
}

impl AsRef<[u8; 8]> for Id {
    #[inline(always)]
    fn as_ref(&self) -> &[u8; 8] {
        unsafe { mem::transmute(&self.0) }
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct TypeId(pub u32);

impl TypeId {
    pub fn from_objectclass(objectclass: &ObjectClass) -> TypeId {
        TypeId(objectclass.protobuf_id())
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<TypeId> {
        if bytes.len() != 4 {
            return Err(Error::Parse);
        }

        let mut id = [0u8; 4];
        id.copy_from_slice(&bytes[0..4]);

        Ok(TypeId(unsafe { mem::transmute(id) }))
    }
}

impl AsRef<[u8; 4]> for TypeId {
    #[inline(always)]
    fn as_ref(&self) -> &[u8; 4] {
        unsafe { mem::transmute(&self.0) }
    }
}

pub struct Entry<'a> {
    pub id: Id,
    pub type_id: TypeId,
    pub data: &'a [u8],
}

impl<'a> Entry<'a> {
    pub fn from_bytes(id: Id, bytes: &[u8]) -> Result<Entry> {
        if bytes.len() < 4 {
            return Err(Error::Parse);
        }

        Ok(Entry {
            id: id,
            type_id: try!(TypeId::from_bytes(&bytes[0..4])),
            data: &bytes[4..],
        })
    }
}
