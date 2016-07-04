extern crate lmdb;
extern crate lmdb_sys;

use std::{result, str};
use std::path::Path;
use lmdb::{Cursor, DUP_SORT, INTEGER_KEY};
use lmdb::Transaction as LmdbTransaction;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Error {
    DbCreateError,
    DbOpenError,
    DbWriteError,
    DbCorruptError,
    ParseError,
    TransactionError,
    PathError,
    NotFoundError,
}

pub type Result<T> = result::Result<T, Error>;

pub struct LmdbAdapter {
    env: lmdb::Environment,
    nodes: lmdb::Database,
    entries: lmdb::Database,
}

pub struct RwTransaction<'a>(lmdb::RwTransaction<'a>);
pub struct RoTransaction<'a>(lmdb::RoTransaction<'a>);

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct Id(u64);

pub struct Node<'a> {
    pub id: Id,
    pub parent_id: Id,
    pub name: &'a str,
}

pub struct Entry<'a> {
    pub node: Node<'a>,
    pub objectclass: &'a str,
}

// Ids are 64-bit integers in host-native byte order
// LMDB has special optimizations for host-native integers as keys
impl Id {
    fn root() -> Id {
        Id(0)
    }

    fn from_bytes(bytes: &[u8]) -> Result<Id> {
        if bytes.len() != 8 {
            return Err(Error::ParseError);
        }

        let mut id = [0u8; 8];
        id.copy_from_slice(&bytes[0..8]);

        Ok(Id(unsafe { std::mem::transmute(id) }))
    }

    fn as_bytes(self) -> [u8; 8] {
        let Id(id) = self;
        unsafe { std::mem::transmute(id) }
    }

    fn next(self) -> Id {
        let Id(id) = self;
        Id(id + 1)
    }
}

impl<'a> Node<'a> {
    fn root() -> Node<'a> {
        Node {
            id: Id::root(),
            parent_id: Id::root(),
            name: "/",
        }
    }

    fn from_parent_id_and_bytes(parent_id: Id, bytes: &[u8]) -> Result<Node> {
        if bytes.len() < 8 {
            return Err(Error::DbCorruptError);
        }

        let id = try!(Id::from_bytes(&bytes[0..8]));
        let name = try!(std::str::from_utf8(&bytes[8..]).map_err(|_err| Error::DbCorruptError));

        Ok(Node {
            id: id,
            parent_id: parent_id,
            name: name,
        })
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(8 + self.name.len());
        bytes.extend_from_slice(&self.id.as_bytes());
        bytes.extend_from_slice(self.name.as_bytes());
        bytes
    }
}

impl LmdbAdapter {
    pub fn create_database(path: &Path) -> Result<LmdbAdapter> {
        let env = try!(lmdb::Environment::new()
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
        let env = try!(lmdb::Environment::new().open(&path).map_err(|_err| Error::DbOpenError));

        let nodes = try!(env.open_db(Some("nodes")).map_err(|_err| Error::DbOpenError));

        let entries = try!(env.open_db(Some("entries")).map_err(|_err| Error::DbOpenError));

        Ok(LmdbAdapter {
            env: env,
            nodes: nodes,
            entries: entries,
        })
    }

    pub fn rw_transaction(&self) -> Result<RwTransaction> {
        match self.env.begin_rw_txn() {
            Ok(txn) => Ok(RwTransaction(txn)),
            Err(_) => Err(Error::TransactionError),
        }
    }

    pub fn ro_transaction(&self) -> Result<RoTransaction> {
        match self.env.begin_ro_txn() {
            Ok(txn) => Ok(RoTransaction(txn)),
            Err(_) => Err(Error::TransactionError),
        }
    }

    pub fn next_available_id<T: Transaction>(&self, txn: &T) -> Result<Id> {
        let cursor = try!(txn.open_ro_cursor(self.nodes)
                             .map_err(|_err| Error::TransactionError));

        let last_id = match cursor.get(None, None, lmdb_sys::MDB_LAST) {
            Ok((id, _)) => Id::from_bytes(id.unwrap()).unwrap(),
            Err(_) => Id::root(),
        };

        Ok(last_id.next())
    }

    pub fn create_entry<'a>(&'a self,
                            txn: &'a mut RwTransaction,
                            id: Id,
                            parent_id: Id,
                            name: &'a str,
                            objectclass: &'a str)
                            -> Result<Entry> {
        let node = Node {
            id: id,
            parent_id: parent_id,
            name: name,
        };

        try!(txn.put(self.nodes,
                     &parent_id.as_bytes(),
                     &node.to_bytes())
                .map_err(|_err| Error::DbWriteError));

        try!(txn.put(self.entries,
                     &id.as_bytes(),
                     &objectclass.as_bytes())
                .map_err(|_err| Error::DbWriteError));

        Ok(Entry {
            node: node,
            objectclass: objectclass,
        })
    }

    pub fn find_entry<'a, T: Transaction>(&'a self, txn: &'a T, path: &str) -> Result<Entry> {
        let node = try!(self.find_node(txn, path));

        let entry_bytes = try!(txn.get(self.entries, &node.id.as_bytes())
                                  .map_err(|_err| Error::DbCorruptError));

        let objectclass = try!(std::str::from_utf8(&entry_bytes)
                                   .map_err(|_err| Error::DbCorruptError));

        Ok(Entry {
            node: node,
            objectclass: objectclass,
        })
    }

    fn find_node<T: Transaction>(&self, txn: &T, path: &str) -> Result<Node> {
        let all_components: Vec<&str> = path.split("/").collect();

        if all_components.is_empty() {
            return Err(Error::PathError);
        }

        let (prefix, components) = all_components.split_first().unwrap();

        // Does the path start with something other than "/"?
        if !prefix.is_empty() {
            return Err(Error::PathError);
        }

        // Perform a hierarchical path lookup
        // TODO: since LMDB is ordered, we could e.g. perform a binary search
        components.iter().fold(Ok(Node::root()), |parent_node, component| {
            let parent_id = try!(parent_node).id;

            let mut cursor = try!(txn.open_ro_cursor(self.nodes)
                                     .map_err(|_err| Error::DbCorruptError));

            let mut child_node = None;

            for (id, node_bytes) in cursor.iter_from(parent_id.as_bytes()) {
                if id != parent_id.as_bytes() {
                    return Err(Error::NotFoundError);
                }

                let node = try!(Node::from_parent_id_and_bytes(parent_id, node_bytes));

                if node.name == *component {
                    child_node = Some(node);
                    break;
                }
            }

            child_node.ok_or(Error::NotFoundError)
        })
    }
}

pub trait Transaction {
    fn get(&self, database: lmdb::Database, key: &[u8]) -> Result<&[u8]>;
    fn commit(self) -> Result<()>;
    fn open_ro_cursor(&self, db: lmdb::Database) -> lmdb::Result<lmdb::RoCursor>;
}

impl<'a> Transaction for RwTransaction<'a> {
    fn get(&self, database: lmdb::Database, key: &[u8]) -> Result<&[u8]> {
        self.0.get(database, &key).map_err(|_err| Error::TransactionError)
    }

    fn commit(self) -> Result<()> {
        self.0.commit().map_err(|_err| Error::TransactionError)
    }

    fn open_ro_cursor(&self, db: lmdb::Database) -> lmdb::Result<lmdb::RoCursor> {
        self.0.open_ro_cursor(db)
    }
}

impl<'a> Transaction for RoTransaction<'a> {
    fn get(&self, database: lmdb::Database, key: &[u8]) -> Result<&[u8]> {
        self.0.get(database, &key).map_err(|_err| Error::TransactionError)
    }

    fn commit(self) -> Result<()> {
        self.0.commit().map_err(|_err| Error::TransactionError)
    }

    fn open_ro_cursor(&self, db: lmdb::Database) -> lmdb::Result<lmdb::RoCursor> {
        self.0.open_ro_cursor(db)
    }
}

impl<'a> RwTransaction<'a> {
    fn put(&mut self, database: lmdb::Database, key: &[u8], data: &[u8]) -> Result<()> {
        self.0.put(database, &key, &data, lmdb::WriteFlags::empty()).map_err(|_err| Error::TransactionError)
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::Path;
    use {LmdbAdapter, Id, Transaction};

    fn create_database() -> LmdbAdapter {
        let path = Path::new("./tmp");

        if !path.exists() {
            panic!("Unable to find repo-local temp path")
        }

        let data = path.join("data.mdb");
        if data.exists() {
            fs::remove_file(data).unwrap()
        }

        let lock = path.join("lock.mdb");
        if lock.exists() {
            fs::remove_file(lock).unwrap()
        }

        LmdbAdapter::create_database(path).unwrap()
    }

    #[test]
    fn test_entry_lookup() {
        let adapter = create_database();

        {
            let mut txn = adapter.rw_transaction().unwrap();

            let domain_id = adapter.next_available_id(&txn).unwrap();
            adapter.create_entry(&mut txn, domain_id, Id::root(), "example.com", "domain").unwrap();

            let hosts_id = domain_id.next();
            adapter.create_entry(&mut txn, hosts_id, domain_id, "hosts", "ou").unwrap();

            let host_id = hosts_id.next();
            adapter.create_entry(&mut txn, host_id, hosts_id, "master.example.com", "host").unwrap();

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
}
