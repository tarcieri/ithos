extern crate lmdb;
extern crate lmdb_sys;

#[cfg(test)]
extern crate tempdir;

use std::path::Path;
use std::str;

use adapter::{Adapter, Transaction};
use server::{Id, Node, Entry, Result, Error};

use self::lmdb::{Environment, Database, Cursor, WriteFlags, DUP_SORT, INTEGER_KEY};
use self::lmdb::Transaction as LmdbTransaction;

pub struct LmdbAdapter {
    env: Environment,
    nodes: Database,
    entries: Database,
}

pub struct RwTransaction<'a>(self::lmdb::RwTransaction<'a>);
pub struct RoTransaction<'a>(self::lmdb::RoTransaction<'a>);

impl LmdbAdapter {
    pub fn create_database(path: &Path) -> Result<LmdbAdapter> {
        let env = try!(Environment::new()
            .set_max_dbs(8)
            .open_with_permissions(&path, 0o600)
            .map_err(|_err| Error::DbCreateError));

        let nodes = try!(env.create_db(Some("nodes"), INTEGER_KEY | DUP_SORT)
            .map_err(|_err| Error::DbCreateError));

        let entries = try!(env.create_db(Some("entries"), INTEGER_KEY)
            .map_err(|_err| Error::DbCreateError));

        Ok(LmdbAdapter {
            env: env,
            nodes: nodes,
            entries: entries,
        })
    }

    pub fn open_database(path: &Path) -> Result<LmdbAdapter> {
        let env = try!(Environment::new()
            .open(&path)
            .map_err(|_err| Error::DbOpenError));

        let nodes = try!(env.open_db(Some("nodes")).map_err(|_err| Error::DbOpenError));

        let entries = try!(env.open_db(Some("entries")).map_err(|_err| Error::DbOpenError));

        Ok(LmdbAdapter {
            env: env,
            nodes: nodes,
            entries: entries,
        })
    }

    fn find_node<'a, T: Transaction<lmdb::Database>>(&'a self,
                                                     txn: &'a T,
                                                     path: &str)
                                                     -> Result<Node> {
        let all_components: Vec<&str> = path.split("/").collect();

        if all_components.is_empty() {
            return Err(Error::PathError);
        }

        let (prefix, components) = all_components.split_first().unwrap();

        // Does the path start with something other than "/"?
        if !prefix.is_empty() {
            return Err(Error::PathError);
        }

        components.iter().fold(Ok(Node::root()), |parent_node, component| {
            self.find_child_node(txn, try!(parent_node).id, component)
        })
    }

    fn find_child_node<'a, T: Transaction<lmdb::Database>>(&'a self,
                                                           txn: &'a T,
                                                           parent_id: Id,
                                                           name: &str)
                                                           -> Result<Node> {
        let node_bytes = try!(txn.find(self.nodes, &parent_id.as_bytes(), |node_bytes| {
            match Node::from_parent_id_and_bytes(parent_id, node_bytes) {
                Ok(node) => node.name == name,
                _ => false,
            }
        }));

        Node::from_parent_id_and_bytes(parent_id, node_bytes)
    }
}

impl<'a> Adapter<'a, lmdb::Database, RoTransaction<'a>, RwTransaction<'a>> for LmdbAdapter {
    fn rw_transaction(&'a self) -> Result<RwTransaction<'a>> {
        match self.env.begin_rw_txn() {
            Ok(txn) => Ok(RwTransaction(txn)),
            Err(_) => Err(Error::TransactionError),
        }
    }

    fn ro_transaction(&'a self) -> Result<RoTransaction<'a>> {
        match self.env.begin_ro_txn() {
            Ok(txn) => Ok(RoTransaction(txn)),
            Err(_) => Err(Error::TransactionError),
        }
    }

    fn next_available_id(&self, txn: &RwTransaction) -> Result<Id> {
        let cursor = try!(txn.0
            .open_ro_cursor(self.nodes)
            .map_err(|_err| Error::TransactionError));

        let last_id = match cursor.get(None, None, lmdb_sys::MDB_LAST) {
            Ok((id, _)) => Id::from_bytes(id.unwrap()).unwrap(),
            Err(_) => Id::root(),
        };

        Ok(last_id.next())
    }

    fn create_entry<'b>(&'b self,
                        txn: &'b mut RwTransaction,
                        id: Id,
                        parent_id: Id,
                        name: &'b str,
                        objectclass: &'b str)
                        -> Result<Entry> {
        if txn.get(self.entries, &id.as_bytes()) != Err(Error::NotFoundError) {
            return Err(Error::DuplicateEntryError);
        }

        if txn.get(self.nodes, &parent_id.as_bytes()) != Err(Error::NotFoundError) &&
           self.find_child_node(txn, parent_id, name) != Err(Error::NotFoundError) {
            return Err(Error::DuplicateEntryError);
        }

        let node = Node {
            id: id,
            parent_id: parent_id,
            name: name,
        };

        try!(txn.put(self.nodes, &parent_id.as_bytes(), &node.to_bytes())
            .map_err(|_err| Error::DbWriteError));

        try!(txn.put(self.entries, &id.as_bytes(), &objectclass.as_bytes())
            .map_err(|_err| Error::DbWriteError));

        Ok(Entry {
            node: node,
            objectclass: objectclass,
        })
    }

    fn find_entry<'b, T: Transaction<lmdb::Database>>(&'b self,
                                                      txn: &'b T,
                                                      path: &str)
                                                      -> Result<Entry> {
        let node = try!(self.find_node(txn, path));

        let entry_bytes = try!(txn.get(self.entries, &node.id.as_bytes())
            .map_err(|_err| Error::DbCorruptError));

        let objectclass = try!(str::from_utf8(&entry_bytes).map_err(|_err| Error::DbCorruptError));

        Ok(Entry {
            node: node,
            objectclass: objectclass,
        })
    }
}

// TODO: since LMDB is ordered, we could e.g. perform a binary search for find
macro_rules! impl_transaction (($newtype:ident) => (
    impl<'a> Transaction<lmdb::Database> for $newtype<'a> {
        fn get(&self, database: Database, key: &[u8]) -> Result<&[u8]> {
            self.0.get(database, &key).map_err(|_err| Error::NotFoundError)
        }

        fn find<P>(&self, db: Database, key: &[u8], predicate: P) -> Result<&[u8]>
            where P: Fn(&[u8]) -> bool
        {
            let mut cursor = try!(self.0
                                      .open_ro_cursor(db)
                                      .map_err(|_err| Error::TransactionError));

            let mut result = None;

            for (cursor_key, value) in cursor.iter_from(key) {
                if cursor_key != key {
                    return Err(Error::NotFoundError);
                }

                if predicate(value) {
                    result = Some(value);
                    break;
                }
            }

            result.ok_or(Error::NotFoundError)
        }

        fn commit(self) -> Result<()> {
            self.0.commit().map_err(|_err| Error::TransactionError)
        }
    }
));

impl_transaction!(RwTransaction);
impl_transaction!(RoTransaction);

impl<'a> RwTransaction<'a> {
    fn put(&mut self, database: Database, key: &[u8], data: &[u8]) -> Result<()> {
        self.0
            .put(database, &key, &data, WriteFlags::empty())
            .map_err(|_err| Error::TransactionError)
    }
}

#[cfg(test)]
mod tests {
    use adapter::{Adapter, Transaction};
    use server::{Id, Error};
    use lmdb_adapter::LmdbAdapter;
    use lmdb_adapter::tempdir::TempDir;

    fn create_database() -> LmdbAdapter {
        let dir = TempDir::new("ithos-test").unwrap();
        LmdbAdapter::create_database(dir.path()).unwrap()
    }

    #[test]
    fn test_entry_lookup() {
        let adapter = create_database();

        {
            let mut txn = adapter.rw_transaction().unwrap();

            let domain_id = adapter.next_available_id(&txn).unwrap();
            adapter.create_entry(&mut txn, domain_id, Id::root(), "example.com", "domain")
                .unwrap();

            let hosts_id = domain_id.next();
            adapter.create_entry(&mut txn, hosts_id, domain_id, "hosts", "ou").unwrap();

            let host_id = hosts_id.next();
            adapter.create_entry(&mut txn, host_id, hosts_id, "master.example.com", "host")
                .unwrap();

            txn.commit().unwrap();
        }

        {
            let txn = adapter.ro_transaction().unwrap();

            {
                let entry = adapter.find_entry(&txn, "/example.com/hosts/master.example.com")
                    .unwrap();

                assert_eq!(entry.node.name, "master.example.com");
                assert_eq!(entry.objectclass, "host");
            }

            txn.commit().unwrap();
        }
    }

    #[test]
    fn test_duplicate_entry_id() {
        let adapter = create_database();

        let mut txn = adapter.rw_transaction().unwrap();

        let domain_id = adapter.next_available_id(&txn).unwrap();
        adapter.create_entry(&mut txn, domain_id, Id::root(), "example.com", "domain").unwrap();

        let result = adapter.create_entry(&mut txn, domain_id, Id::root(), "another.com", "domain");
        assert_eq!(result, Err(Error::DuplicateEntryError));
    }

    #[test]
    fn test_duplicate_entry_name() {
        let adapter = create_database();

        let mut txn = adapter.rw_transaction().unwrap();

        let domain_id = adapter.next_available_id(&txn).unwrap();
        adapter.create_entry(&mut txn, domain_id, Id::root(), "example.com", "domain").unwrap();

        let result = adapter.create_entry(&mut txn, domain_id.next(), Id::root(), "example.com", "domain");
        assert_eq!(result, Err(Error::DuplicateEntryError));
    }
}
