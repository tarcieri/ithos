//! id.rs: Type-specific identifiers
//!
//! A dumping ground for ID types. Well, only two at present.
//!

use block::Block;
use error::{Error, Result};
use objecthash::{self, ObjectHash, ObjectHasher};
use std::mem;

const DIGEST_SIZE: usize = 32;

// Block IDs are presently SHA-256 only
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct BlockId([u8; DIGEST_SIZE]);

impl BlockId {
    // Parent ID of the initial block (256-bits of zero)
    pub fn zero() -> BlockId {
        BlockId([0u8; DIGEST_SIZE])
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<BlockId> {
        if bytes.len() != DIGEST_SIZE {
            return Err(Error::parse(None));
        }

        let mut id = [0u8; DIGEST_SIZE];
        id.copy_from_slice(&bytes[0..DIGEST_SIZE]);

        Ok(BlockId(id))
    }

    pub fn of(block: &Block) -> BlockId {
        BlockId::from_bytes(objecthash::digest(block).as_ref()).unwrap()
    }
}

impl AsRef<[u8]> for BlockId {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl ObjectHash for BlockId {
    fn objecthash<H: ObjectHasher>(&self, hasher: &mut H) {
        self.0.objecthash(hasher);
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct EntryId(u64);

// EntryIds are 64-bit integers in host-native byte order
// LMDB has special optimizations for host-native integers as keys
impl EntryId {
    pub fn root() -> EntryId {
        EntryId(0)
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<EntryId> {
        if bytes.len() != 8 {
            return Err(Error::parse(None));
        }

        let mut id = [0u8; 8];
        id.copy_from_slice(&bytes[0..8]);

        Ok(EntryId(unsafe { mem::transmute(id) }))
    }

    pub fn next(self) -> EntryId {
        EntryId(self.0 + 1)
    }
}

impl AsRef<[u8; 8]> for EntryId {
    #[inline]
    fn as_ref(&self) -> &[u8; 8] {
        unsafe { mem::transmute(&self.0) }
    }
}
