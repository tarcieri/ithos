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

struct Entry<'a> {
    id: u64,
    parent_id: u64,
    name: &'a str,
    objectclass: &'a str,
}

struct Node<'a> {
    id: u64,
    parent_id: u64,
    name: &'a str,
}

impl<'a> Node<'a> {
    fn root() -> Node<'a> {
        Node {
            id: 0,
            parent_id: 0,
            name: "/"
        }
    }

    fn from_bytes_and_parent_id(bytes: &[u8], parent_id: u64) -> Result<Node> {
        if bytes.len() < 8 {
            return Err(Error::DbCorruptError);
        }

        let mut id = [0u8; 8];
        id.copy_from_slice(&bytes[0..8]);

        let name = match std::str::from_utf8(&bytes[8..]) {
            Ok(string) => string,
            _ => return Err(Error::DbCorruptError),
        };

        Ok(Node {
            id: LmdbAdapter::bytes_to_id(id),
            parent_id: parent_id,
            name: name,
        })
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(8 + self.name.len());
        bytes.extend_from_slice(&LmdbAdapter::id_to_bytes(self.id));
        bytes.extend_from_slice(self.name.as_bytes());
        bytes
    }
}

impl LmdbAdapter {
    pub fn create(path: &Path) -> Result<LmdbAdapter> {
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

    pub fn open(path: &Path) -> Result<LmdbAdapter> {
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

    fn id_to_bytes(id: u64) -> [u8; 8] {
        unsafe { std::mem::transmute(id) }
    }

    fn bytes_to_id(bytes: [u8; 8]) -> u64 {
        unsafe { std::mem::transmute(bytes) }
    }

    pub fn rw_transaction(&self) -> Result<RwTransaction> {
        match self.env.begin_rw_txn() {
            Ok(txn) => Ok(RwTransaction { lmdb_txn: txn }),
            Err(_) => Err(Error::TransactionError),
        }
    }

    pub fn next_available_id(&self, txn: &mut RwTransaction) -> Result<u64> {
        let cursor = match txn.lmdb_txn.open_rw_cursor(self.nodes) {
            Ok(c) => c,
            Err(_) => return Err(Error::TransactionError),
        };

        let last_id = match cursor.get(None, None, lmdb_sys::MDB_LAST) {
            Ok((id, _)) => {
                let mut id_bytes = [0u8; 8];
                id_bytes.copy_from_slice(id.unwrap()); // TODO: don't panic
                LmdbAdapter::bytes_to_id(id_bytes)
            }
            Err(_) => 0, // 0 is reserved for root
        };

        Ok(last_id + 1)
    }

    pub fn create_entry<'a>(&'a self,
                            txn: &mut RwTransaction,
                            id: u64,
                            parent_id: u64,
                            name: &'a str,
                            objectclass: &'a str)
                            -> Result<Entry> {
        let ref mut lmdb_txn = txn.lmdb_txn;
        let id_bytes = LmdbAdapter::id_to_bytes(id);

        let mut path_info = Vec::with_capacity(8 + name.len());
        path_info.extend_from_slice(&id_bytes);
        path_info.extend_from_slice(name.as_bytes());

        match lmdb_txn.put(self.nodes,
                           &LmdbAdapter::id_to_bytes(parent_id),
                           &path_info,
                           lmdb::WriteFlags::empty()) {
            Ok(_) => (),
            Err(_) => return Err(Error::DbWriteError),
        }

        match lmdb_txn.put(self.entries,
                           &id_bytes,
                           &objectclass,
                           lmdb::WriteFlags::empty()) {
            Ok(_) => (),
            Err(_) => return Err(Error::DbWriteError),
        }

        Ok(Entry {
            id: id,
            parent_id: parent_id,
            name: name,
            objectclass: objectclass,
        })
    }

    pub fn find_entry<'a>(&'a self, txn: &'a mut RwTransaction, path: &str) -> Result<Entry> {
        let node = try!(self.find_node(txn, path));

        let entry_bytes = match txn.lmdb_txn.get(self.entries, &LmdbAdapter::id_to_bytes(node.id)) {
            Ok(bytes) => bytes,
            Err(_) => return Err(Error::DbCorruptError),
        };

        let objectclass = match std::str::from_utf8(&entry_bytes) {
            Ok(string) => string,
            _ => return Err(Error::DbCorruptError),
        };

        Ok(Entry {
            id: node.id,
            parent_id: node.parent_id,
            name: node.name,
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
            let parent_id_bytes = LmdbAdapter::id_to_bytes(parent_id);

            let mut cursor = match txn.lmdb_txn.open_rw_cursor(self.nodes) {
                Ok(c) => c,
                Err(_) => return Err(Error::TransactionError),
            };

            let mut child_node = None;

            for (id, node_bytes) in cursor.iter_from(parent_id_bytes) {
                if id != parent_id_bytes {
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
    let adapter = LmdbAdapter::create(path).unwrap();

    {
        let mut txn = adapter.rw_transaction().unwrap();
        let id = adapter.next_available_id(&mut txn).unwrap();
        let domain = adapter.create_entry(&mut txn, id, 0, "example.com", "domain").unwrap();
        let hosts = adapter.create_entry(&mut txn, id + 1, domain.id, "hosts", "ou").unwrap();
        let host = adapter.create_entry(&mut txn, id + 2, hosts.id, "master.example.com", "host")
                          .unwrap();

        txn.commit().unwrap();
    }

    {
        let mut txn = adapter.rw_transaction().unwrap();
        let entry = adapter.find_entry(&mut txn, "/example.com/hosts/master.example.com").unwrap();

        assert_eq!(entry.name, "master.example.com");
        assert_eq!(entry.objectclass, "host");
    }
}
