//! id.rs: Type-specific identifiers
//!
//! A dumping ground for ID types. Well, only two at present.
//!

use block::Block;
use byteorder::{ByteOrder, NativeEndian};
use error::{Error, Result};
use objecthash::{self, ObjectHash, ObjectHasher};
use std::mem;

const DIGEST_SIZE: usize = 32;

/// Identifiers for blocks. All `BlockID` values are presently SHA-256 only
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct BlockId([u8; DIGEST_SIZE]);

impl BlockId {
    /// Parent ID of the initial block (256-bits of zero)
    pub fn zero() -> BlockId {
        BlockId([0u8; DIGEST_SIZE])
    }

    /// Create a block ID from a serialized
    pub fn from_bytes(bytes: &[u8]) -> Result<BlockId> {
        if bytes.len() != DIGEST_SIZE {
            return Err(Error::parse(None));
        }

        let mut id = [0u8; DIGEST_SIZE];
        id.copy_from_slice(&bytes[0..DIGEST_SIZE]);

        Ok(BlockId(id))
    }

    /// Return the `BlockID` of the given block
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

/// An `EntryId` is a 64-bit integer in host-native byte order.
/// LMDB has special optimizations for host-native integers as keys.
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct EntryId(u64);

impl EntryId {
    /// The root entry ("/") in the directory (ala LDAP root DSE)
    pub fn root() -> EntryId {
        EntryId(0)
    }

    /// Deserialize an `EntryID` from a host native byte format
    pub fn from_bytes(bytes: &[u8]) -> Result<EntryId> {
        if bytes.len() != 8 {
            return Err(Error::parse(None));
        }

        Ok(EntryId(NativeEndian::read_u64(bytes)))
    }

    /// Obtain the next sequential EntryID after this one
    pub fn next(&self) -> EntryId {
        EntryId(self.0 + 1)
    }
}

impl AsRef<[u8; 8]> for EntryId {
    // TODO: get rid of transmute
    #[inline]
    #[allow(unsafe_code)]
    fn as_ref(&self) -> &[u8; 8] {
        unsafe { mem::transmute(&self.0) }
    }
}
