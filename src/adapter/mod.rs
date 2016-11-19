pub mod lmdb;


use block::{self, Block};
use direntry::DirEntry;
use entry::{self, Entry};
use error::Result;
use metadata::Metadata;
use path;
use std::marker::Sized;
use std::path::Path as StdPath;

pub trait Transaction {
    type D;

    fn commit(self) -> Result<()>;
    fn get(&self, db: Self::D, key: &[u8]) -> Result<&[u8]>;
    fn find<P>(&self, db: Self::D, key: &[u8], predicate: P) -> Result<&[u8]>
        where P: Fn(&[u8]) -> bool;
}

pub trait Adapter<'a> {
    type D;
    type R: Transaction<D = Self::D>;
    type W: Transaction<D = Self::D>;

    fn create_database(path: &StdPath) -> Result<Self> where Self: Sized;
    fn open_database(path: &StdPath) -> Result<Self> where Self: Sized;

    fn ro_transaction(&'a self) -> Result<Self::R>;
    fn rw_transaction(&'a self) -> Result<Self::W>;

    fn next_free_entry_id(&self, txn: &Self::W) -> Result<entry::Id>;

    fn add_block<'t>(&'t self, txn: &'t mut Self::W, block: &Block) -> Result<()>;
    fn current_block_id<'t, T>(&'t self, txn: &'t T) -> Result<block::Id>
        where T: Transaction<D = Self::D>;
    fn add_entry<'t>(&'t self,
                     txn: &'t mut Self::W,
                     entry: &Entry,
                     name: &'t str,
                     parent_id: entry::Id,
                     metadata: &Metadata)
                     -> Result<DirEntry>;
    fn find_direntry<'t, T>(&'t self, txn: &'t T, path: &path::Path) -> Result<DirEntry>
        where T: Transaction<D = Self::D>;
    fn find_metadata<'t, T>(&'t self, txn: &'t T, id: &entry::Id) -> Result<Metadata>
        where T: Transaction<D = Self::D>;
    fn find_entry<'t, T>(&'t self, txn: &'t T, id: &entry::Id) -> Result<Entry>
        where T: Transaction<D = Self::D>;
}
