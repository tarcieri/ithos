extern crate lmdb;
extern crate lmdb_sys;

use std::{result, str};
use std::path::Path;
use lmdb::{Transaction, Cursor, DUP_SORT, INTEGER_KEY};

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

struct LmdbAdapter {
    env: lmdb::Environment,
    nodes: lmdb::Database,
    entries: lmdb::Database,
}

struct RwTransaction<'a> {
    lmdb_txn: lmdb::RwTransaction<'a>,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
struct Id(u64);

struct Node<'a> {
    id: Id,
    parent_id: Id,
    name: &'a str,
}

struct Entry<'a> {
    node: Node<'a>,
    objectclass: &'a str,
}

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

    fn from_bytes_and_parent_id(bytes: &[u8], parent_id: Id) -> Result<Node> {
        if bytes.len() < 8 {
            return Err(Error::DbCorruptError);
        }

        let id = try!(Id::from_bytes(&bytes[0..8]));

        let name = match std::str::from_utf8(&bytes[8..]) {
            Ok(string) => string,
            _ => return Err(Error::DbCorruptError),
        };

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
        let env = match lmdb::Environment::new()
                            .set_max_dbs(8)
                            .open_with_permissions(&path, 0o600) {
            Ok(e) => e,
            Err(_) => return Err(Error::DbCreateError),
        };

        let nodes = match env.create_db(Some("nodes"), INTEGER_KEY | DUP_SORT) {
            Ok(db) => db,
            Err(_) => return Err(Error::DbCreateError),
        };

        let entries = match env.create_db(Some("entries"), INTEGER_KEY) {
            Ok(db) => db,
            Err(_) => return Err(Error::DbCreateError),
        };

        Ok(LmdbAdapter {
            env: env,
            nodes: nodes,
            entries: entries,
        })
    }

    pub fn open_database(path: &Path) -> Result<LmdbAdapter> {
        let env = match lmdb::Environment::new().open(&path) {
            Ok(e) => e,
            Err(_) => return Err(Error::DbOpenError),
        };

        let nodes = match env.open_db(Some("nodes")) {
            Ok(e) => e,
            Err(_) => return Err(Error::DbOpenError),
        };

        let entries = match env.open_db(Some("entries")) {
            Ok(e) => e,
            Err(_) => return Err(Error::DbOpenError),
        };

        Ok(LmdbAdapter {
            env: env,
            nodes: nodes,
            entries: entries,
        })
    }

    pub fn rw_transaction(&self) -> Result<RwTransaction> {
        match self.env.begin_rw_txn() {
            Ok(txn) => Ok(RwTransaction { lmdb_txn: txn }),
            Err(_) => Err(Error::TransactionError),
        }
    }

    pub fn next_available_id(&self, txn: &mut RwTransaction) -> Result<Id> {
        let cursor = match txn.lmdb_txn.open_rw_cursor(self.nodes) {
            Ok(c) => c,
            Err(_) => return Err(Error::TransactionError),
        };

        let last_id = match cursor.get(None, None, lmdb_sys::MDB_LAST) {
            Ok((id, _)) => Id::from_bytes(id.unwrap()).unwrap(),
            Err(_) => Id::root(),
        };

        Ok(last_id.next())
    }

    pub fn create_entry<'a>(&'a self,
                            txn: &mut RwTransaction,
                            id: Id,
                            parent_id: Id,
                            name: &'a str,
                            objectclass: &'a str)
                            -> Result<Entry> {
        let ref mut lmdb_txn = txn.lmdb_txn;

        let node = Node {
            id: id,
            parent_id: parent_id,
            name: name,
        };

        match lmdb_txn.put(self.nodes,
                           &parent_id.as_bytes(),
                           &node.to_bytes(),
                           lmdb::WriteFlags::empty()) {
            Ok(_) => (),
            Err(_) => return Err(Error::DbWriteError),
        }

        match lmdb_txn.put(self.entries,
                           &id.as_bytes(),
                           &objectclass,
                           lmdb::WriteFlags::empty()) {
            Ok(_) => (),
            Err(_) => return Err(Error::DbWriteError),
        }

        Ok(Entry {
            node: node,
            objectclass: objectclass,
        })
    }

    pub fn find_entry<'a>(&'a self, txn: &'a mut RwTransaction, path: &str) -> Result<Entry> {
        let node = try!(self.find_node(txn, path));

        let entry_bytes = match txn.lmdb_txn.get(self.entries, &node.id.as_bytes()) {
            Ok(bytes) => bytes,
            Err(_) => return Err(Error::DbCorruptError),
        };

        let objectclass = match std::str::from_utf8(&entry_bytes) {
            Ok(string) => string,
            _ => return Err(Error::DbCorruptError),
        };

        Ok(Entry {
            node: node,
            objectclass: objectclass,
        })
    }

    fn find_node(&self, txn: &mut RwTransaction, path: &str) -> Result<Node> {
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

            let mut cursor = match txn.lmdb_txn.open_rw_cursor(self.nodes) {
                Ok(c) => c,
                Err(_) => return Err(Error::TransactionError),
            };

            let mut child_node = None;

            for (id, node_bytes) in cursor.iter_from(parent_id.as_bytes()) {
                if id != parent_id.as_bytes() {
                    return Err(Error::NotFoundError);
                }

                let node = try!(Node::from_bytes_and_parent_id(node_bytes, parent_id));

                if node.name == *component {
                    child_node = Some(node);
                    break;
                }
            }

            match child_node {
                Some(node) => Ok(node),
                None => return Err(Error::NotFoundError),
            }
        })
    }
}

impl<'a> RwTransaction<'a> {
    pub fn commit(self) -> Result<()> {
        match self.lmdb_txn.commit() {
            Ok(_) => Ok(()),
            Err(_) => return Err(Error::TransactionError),
        }
    }
}

#[test]
fn test_entry_lookup() {
    let path = Path::new("./tmp");
    let adapter = LmdbAdapter::create_database(path).unwrap();

    {
        let mut txn = adapter.rw_transaction().unwrap();

        let domain_id = adapter.next_available_id(&mut txn).unwrap();
        let domain = adapter.create_entry(&mut txn, domain_id, Id::root(), "example.com", "domain")
                            .unwrap();

        let hosts_id = domain_id.next();
        let hosts = adapter.create_entry(&mut txn, hosts_id, domain_id, "hosts", "ou").unwrap();

        let host_id = hosts_id.next();
        let host = adapter.create_entry(&mut txn, host_id, hosts_id, "master.example.com", "host")
                          .unwrap();

        txn.commit().unwrap();
    }

    {
        let mut txn = adapter.rw_transaction().unwrap();
        let entry = adapter.find_entry(&mut txn, "/example.com/hosts/master.example.com").unwrap();

        assert_eq!(entry.node.name, "master.example.com");
        assert_eq!(entry.objectclass, "host");
    }
}
