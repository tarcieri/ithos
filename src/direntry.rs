//! direntry.rs: Named association between an entry and its parent
//!
//! In practice this leverages LMDB's multi-entry-per-key functionality to store children
//! in a directory hierarchy
//!

use error::{Error, Result};
use id::EntryId;
use std::str;

/// Entries in the directory tree which map names to identifiers
#[derive(Debug, Eq, PartialEq)]
pub struct DirEntry<'a> {
    /// ID of the entry that corresponds to this node in the directory tree
    pub id: EntryId,

    /// Parent ID of this entry
    pub parent_id: EntryId,

    /// Name of this entry within the directory tree
    pub name: &'a str,
}

impl<'a> DirEntry<'a> {
    /// Root entry in the directory tree (i.e. "/")
    pub fn root() -> DirEntry<'a> {
        DirEntry {
            id: EntryId::root(),
            parent_id: EntryId::root(),
            name: "/",
        }
    }

    /// Parse a serialized directory entry into a DirEntry structure
    pub fn new(parent_id: EntryId, bytes: &[u8]) -> Result<DirEntry> {
        if bytes.len() < 8 {
            return Err(Error::parse(None));
        }

        let id = try!(EntryId::from_bytes(&bytes[0..8]));
        let name = try!(str::from_utf8(&bytes[8..]).map_err(|_| Error::parse(None)));

        Ok(DirEntry {
            id: id,
            parent_id: parent_id,
            name: name,
        })
    }

    /// Serialize a DirEntry to its byte representation
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(8 + self.name.len());
        bytes.extend_from_slice(self.id.as_ref());
        bytes.extend_from_slice(self.name.as_bytes());
        bytes
    }
}
