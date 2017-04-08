//! adapter/mod.rs: A not-fully-fleshed-out "aspirational" abstraction for multiple storage adapters
//!
//! This API is heavily inspired by LMDB's multi-reader, single-writer model with separate
//! read-only and read-write transactions, however it encapsulates LMDB internals and avoids
//! exposing any LMDB-related types except as associated types (i.e. the database type).
//!
//! That said, this is probably not close to the ideal API for an abstract storage adapter, and
//! much logic that's in lmdb.rs right now should probably get hoisted out first.
//!

#[cfg(feature = "lmdb")]
pub mod lmdb;

// Allow adapters to define a trait of their internal transaction methods

#[cfg(feature = "lmdb")]
pub use self::lmdb::AdapterTransaction;

use block::Block;
use direntry::DirEntry;
use entry::SerializedEntry;
use errors::*;
use id::{BlockId, EntryId};
use metadata::Metadata;
use path;
use std::marker::Sized;
use std::path::Path as StdPath;

/// All access to the underlying storage system is transactional
pub trait Transaction: AdapterTransaction {
    /// Commit this transaction (performing writes if this is a read-write transaction
    fn commit(self) -> Result<()>;
}

/// Abstract (but still roughly LMDB-shaped) adapter interface with high-level APIs to work with
/// the various types of data in an ithos directory
pub trait Adapter<'a> {
    /// Read-only transaction type
    type R: Transaction;

    /// Read-write transaction type
    type W: Transaction;

    /// Create a new database at the given path
    fn create_database(path: &StdPath) -> Result<Self> where Self: Sized;

    /// Open an existing database at the given path
    fn open_database(path: &StdPath) -> Result<Self> where Self: Sized;

    /// Begin a read-only transaction
    fn ro_transaction(&'a self) -> Result<Self::R>;

    /// Begin a read-write transaction
    fn rw_transaction(&'a self) -> Result<Self::W>;

    /// Obtain the next available entry ID
    fn next_free_entry_id(&self, txn: &Self::W) -> Result<EntryId>;

    /// Add a block to the database (NOTE: just stores the block, doesn't validate/process it)
    fn add_block<'t>(&'t self, txn: &'t mut Self::W, block: &Block) -> Result<()>;

    /// Obtain the current block ID
    fn current_block_id<'t, T>(&'t self, txn: &'t T) -> Result<BlockId> where T: Transaction;

    /// Add an entry to the database
    fn add_entry<'t>(&'t self,
                     txn: &'t mut Self::W,
                     entry: &SerializedEntry,
                     name: &'t str,
                     parent_id: EntryId,
                     metadata: &Metadata)
                     -> Result<DirEntry>;

    /// Find the directory entry (including entry ID) under the given path
    fn find_direntry<'t, T>(&'t self, txn: &'t T, path: &path::Path) -> Result<DirEntry>
        where T: Transaction;

    /// Find the metadata associated with a given entry ID
    fn find_metadata<'t, T>(&'t self, txn: &'t T, id: &EntryId) -> Result<Metadata>
        where T: Transaction;

    /// Find the serialized entry under a given entry ID
    fn find_entry<'t, T>(&'t self, txn: &'t T, id: &EntryId) -> Result<SerializedEntry>
        where T: Transaction;
}
