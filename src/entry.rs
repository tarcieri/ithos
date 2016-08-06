use std::mem;

use error::{Error, Result};

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
