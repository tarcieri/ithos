use block::Block;
use direntry::DirEntry;
use entry::Entry;
use error::Result;
use id::Id;
use objectclass::ObjectClass;
use path::Path;

pub trait Transaction<D> {
    fn commit(self) -> Result<()>;
    fn get(&self, database: D, key: &[u8]) -> Result<&[u8]>;
    fn find<P>(&self, db: D, key: &[u8], predicate: P) -> Result<&[u8]> where P: Fn(&[u8]) -> bool;
}

pub trait Adapter<'a, D, R: Transaction<D>, W: Transaction<D>> {
    fn ro_transaction(&'a self) -> Result<R>;
    fn rw_transaction(&'a self) -> Result<W>;
    fn next_available_id(&self, txn: &W) -> Result<Id>;
    fn add_block<'b>(&'b self, txn: &'b mut W, block: &Block) -> Result<()>;
    fn add_entry<'b>(&'b self,
                     txn: &'b mut W,
                     id: Id,
                     parent_id: Id,
                     name: &'b str,
                     objectclass: ObjectClass)
                     -> Result<Entry>;
    fn find_direntry<'b, T: Transaction<D>>(&'b self, txn: &'b T, path: &Path) -> Result<DirEntry>;
    fn find_entry<'b, T: Transaction<D>>(&'b self, txn: &'b T, path: &Path) -> Result<Entry>;
}
