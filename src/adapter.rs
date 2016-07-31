use objectclass::ObjectClass;
use error::Result;
use server::{Id, Node, Entry, Path};

pub trait Transaction<D> {
    fn commit(self) -> Result<()>;
    fn get(&self, database: D, key: &[u8]) -> Result<&[u8]>;
    fn find<P>(&self, db: D, key: &[u8], predicate: P) -> Result<&[u8]> where P: Fn(&[u8]) -> bool;
}

pub trait Adapter<'a, D, R: Transaction<D>, W: Transaction<D>> {
    fn ro_transaction(&'a self) -> Result<R>;
    fn rw_transaction(&'a self) -> Result<W>;
    fn next_available_id(&self, txn: &W) -> Result<Id>;
    fn add_entry<'b>(&'b self,
                     txn: &'b mut W,
                     id: Id,
                     parent_id: Id,
                     name: &'b str,
                     objectclass: ObjectClass)
                     -> Result<Entry>;
    fn find_node<'b, T: Transaction<D>>(&'b self, txn: &'b T, path: &Path) -> Result<Node>;
    fn find_entry<'b, T: Transaction<D>>(&'b self, txn: &'b T, path: &Path) -> Result<Entry>;
}
