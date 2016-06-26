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
    TransactionError,
}

pub type Result<T> = result::Result<T, Error>;

struct LmdbAdapter {
    env: lmdb::Environment,
    path2id: lmdb::Database,
    id2entry: lmdb::Database,
}

struct RwTransaction<'a> {
    lmdb_txn: lmdb::RwTransaction<'a>,
}

struct Node<'a> {
    id: u64,
    parent_id: u64,
    name: &'a str,
    objectclass: &'a str,
}

impl LmdbAdapter {
    pub fn create(path: &Path) -> Result<LmdbAdapter> {
        let env = match lmdb::Environment::new()
                            .set_max_dbs(8)
                            .open_with_permissions(&path, 0o600) {
            Ok(e) => e,
            Err(_) => return Err(Error::DbCreateError),
        };

        let path2id = match env.create_db(Some("path2id"), INTEGER_KEY | DUP_SORT) {
            Ok(db) => db,
            Err(_) => return Err(Error::DbCreateError),
        };

        let id2entry = match env.create_db(Some("id2entry"), INTEGER_KEY) {
            Ok(db) => db,
            Err(_) => return Err(Error::DbCreateError),
        };

        Ok(LmdbAdapter {
            env: env,
            path2id: path2id,
            id2entry: id2entry,
        })
    }

    pub fn open(path: &Path) -> Result<LmdbAdapter> {
        let env = match lmdb::Environment::new().open(&path) {
            Ok(e) => e,
            Err(_) => return Err(Error::DbOpenError),
        };

        let path2id = match env.open_db(Some("path2id")) {
            Ok(e) => e,
            Err(_) => return Err(Error::DbOpenError),
        };

        let id2entry = match env.open_db(Some("id2entry")) {
            Ok(e) => e,
            Err(_) => return Err(Error::DbOpenError),
        };

        Ok(LmdbAdapter {
            env: env,
            path2id: path2id,
            id2entry: id2entry,
        })
    }

    fn id2bytes(id: u64) -> [u8; 8] {
        unsafe { std::mem::transmute(id) }
    }

    fn bytes2id(bytes: [u8; 8]) -> u64 {
        unsafe { std::mem::transmute(bytes) }
    }

    pub fn rw_transaction(&self) -> Result<RwTransaction> {
        match self.env.begin_rw_txn() {
            Ok(txn) => Ok(RwTransaction { lmdb_txn: txn }),
            Err(_) => Err(Error::TransactionError),
        }
    }

    pub fn next_available_id(&self, txn: &mut RwTransaction) -> Result<u64> {
        let cursor = match txn.lmdb_txn.open_rw_cursor(self.path2id) {
            Ok(c) => c,
            Err(_) => return Err(Error::TransactionError),
        };

        let last_id = match cursor.get(None, None, lmdb_sys::MDB_LAST) {
            Ok((key, _)) => {
                let mut key_bytes = [0u8; 8];
                key_bytes.copy_from_slice(key.unwrap()); // TODO: don't panic
                LmdbAdapter::bytes2id(key_bytes)
            }
            Err(_) => 0, // 0 is reserved for root
        };

        Ok(last_id + 1)
    }

    pub fn add_node<'a>(&'a self,
                        txn: &mut RwTransaction,
                        id: u64,
                        parent_id: u64,
                        name: &'a str,
                        objectclass: &'a str)
                        -> Result<Node> {
        let ref mut lmdb_txn = txn.lmdb_txn;
        let id_bytes = LmdbAdapter::id2bytes(id);

        let mut path_info = Vec::with_capacity(8 + name.len());
        path_info.extend_from_slice(&id_bytes);
        path_info.extend_from_slice(name.as_bytes());

        match lmdb_txn.put(self.path2id,
                           &LmdbAdapter::id2bytes(parent_id),
                           &path_info,
                           lmdb::WriteFlags::empty()) {
            Ok(_) => (),
            Err(_) => return Err(Error::DbWriteError),
        }

        match lmdb_txn.put(self.id2entry,
                           &id_bytes,
                           &objectclass,
                           lmdb::WriteFlags::empty()) {
            Ok(_) => (),
            Err(_) => return Err(Error::DbWriteError),
        }

        Ok(Node {
            id: id,
            parent_id: parent_id,
            name: name,
            objectclass: objectclass,
        })
    }
}

impl<'a> RwTransaction<'a> {
    pub fn commit(self) -> () {
        self.lmdb_txn.commit().unwrap()
    }
}

#[test]
fn test_stuff() {
    let path = Path::new("./tmp");
    let adapter = LmdbAdapter::create(path).unwrap();
    let mut txn = adapter.rw_transaction().unwrap();
    let id = adapter.next_available_id(&mut txn).unwrap();
    let node = adapter.add_node(&mut txn, id, 0, "example.com", "domain").unwrap();
    txn.commit();
}
