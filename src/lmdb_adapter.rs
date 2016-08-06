extern crate lmdb;
extern crate lmdb_sys;

#[cfg(test)]
extern crate tempdir;

use std::{self, str};

use buffoon;
use self::lmdb::{Environment, Database, DatabaseFlags, Cursor, WriteFlags, DUP_SORT, INTEGER_KEY};
use self::lmdb::Transaction as LmdbTransaction;

use adapter::{Adapter, Transaction};
use block::Block;
use direntry::DirEntry;
use entry::Entry;
use error::{Error, Result};
use id::Id;
use objectclass::ObjectClass;
use path::Path;

pub struct LmdbAdapter {
    env: Environment,
    blocks: Database,
    directories: Database,
    entries: Database,
}

pub struct RwTransaction<'a>(self::lmdb::RwTransaction<'a>);
pub struct RoTransaction<'a>(self::lmdb::RoTransaction<'a>);

impl LmdbAdapter {
    pub fn create_database(path: &std::path::Path) -> Result<LmdbAdapter> {
        let env = try!(Environment::new()
            .set_max_dbs(8)
            .open_with_permissions(&path, 0o600)
            .map_err(|_| Error::DbCreate));

        let blocks = try!(env.create_db(Some("blocks"), DatabaseFlags::empty())
            .map_err(|_| Error::DbCreate));

        let directories = try!(env.create_db(Some("directories"), INTEGER_KEY | DUP_SORT)
            .map_err(|_| Error::DbCreate));

        let entries = try!(env.create_db(Some("entries"), INTEGER_KEY)
            .map_err(|_| Error::DbCreate));

        Ok(LmdbAdapter {
            env: env,
            blocks: blocks,
            directories: directories,
            entries: entries,
        })
    }

    pub fn open_database(path: &std::path::Path) -> Result<LmdbAdapter> {
        let env = try!(Environment::new()
            .open(&path)
            .map_err(|_| Error::DbOpen));

        let blocks = try!(env.open_db(Some("blocks")).map_err(|_| Error::DbOpen));

        let directories = try!(env.open_db(Some("directories")).map_err(|_| Error::DbOpen));

        let entries = try!(env.open_db(Some("entries")).map_err(|_| Error::DbOpen));

        Ok(LmdbAdapter {
            env: env,
            blocks: blocks,
            directories: directories,
            entries: entries,
        })
    }

    fn find_child<'a, T: Transaction<lmdb::Database>>(&'a self,
                                                      txn: &'a T,
                                                      parent_id: Id,
                                                      name: &str)
                                                      -> Result<DirEntry> {
        let direntry_bytes =
            try!(txn.find(self.directories, &parent_id.as_bytes(), |direntry_bytes| {
                match DirEntry::new(parent_id, direntry_bytes) {
                    Ok(direntry) => direntry.name == name,
                    _ => false,
                }
            }));

        DirEntry::new(parent_id, direntry_bytes)
    }
}

impl<'a> Adapter<'a, lmdb::Database, RoTransaction<'a>, RwTransaction<'a>> for LmdbAdapter {
    fn rw_transaction(&'a self) -> Result<RwTransaction<'a>> {
        match self.env.begin_rw_txn() {
            Ok(txn) => Ok(RwTransaction(txn)),
            Err(_) => Err(Error::Transaction),
        }
    }

    fn ro_transaction(&'a self) -> Result<RoTransaction<'a>> {
        match self.env.begin_ro_txn() {
            Ok(txn) => Ok(RoTransaction(txn)),
            Err(_) => Err(Error::Transaction),
        }
    }

    fn next_available_id(&self, txn: &RwTransaction) -> Result<Id> {
        let cursor = try!(txn.0
            .open_ro_cursor(self.directories)
            .map_err(|_| Error::Transaction));

        let last_id = match cursor.get(None, None, lmdb_sys::MDB_LAST) {
            Ok((id, _)) => Id::from_bytes(id.unwrap()).unwrap(),
            Err(_) => Id::root(),
        };

        Ok(last_id.next())
    }

    fn add_block<'b>(&'b self, txn: &'b mut RwTransaction, block: &Block) -> Result<()> {
        // TODO: Don't Panic
        let block_id = block.id.expect("block ID unset");

        let serialized = try!(buffoon::serialize(&block).map_err(|_| Error::Serialize));

        if txn.get(self.blocks, &block_id) != Err(Error::NotFound) {
            return Err(Error::EntryAlreadyExists);
        }

        try!(txn.put(self.blocks, &block_id, &serialized)
            .map_err(|_| Error::DbWrite));

        Ok(())
    }

    fn add_entry<'b>(&'b self,
                     txn: &'b mut RwTransaction,
                     id: Id,
                     parent_id: Id,
                     name: &'b str,
                     objectclass: ObjectClass)
                     -> Result<Entry> {
        if txn.get(self.entries, &id.as_bytes()) != Err(Error::NotFound) {
            return Err(Error::EntryAlreadyExists);
        }

        if txn.get(self.directories, &parent_id.as_bytes()) != Err(Error::NotFound) &&
           self.find_child(txn, parent_id, name) != Err(Error::NotFound) {
            return Err(Error::EntryAlreadyExists);
        }

        let direntry = DirEntry {
            id: id,
            parent_id: parent_id,
            name: name,
        };

        try!(txn.put(self.directories,
                 &parent_id.as_bytes(),
                 &direntry.to_bytes())
            .map_err(|_| Error::DbWrite));

        try!(txn.put(self.entries,
                 &id.as_bytes(),
                 &objectclass.to_string().as_bytes())
            .map_err(|_| Error::DbWrite));

        Ok(Entry {
            direntry: direntry,
            objectclass: objectclass,
        })
    }

    fn find_direntry<'b, T: Transaction<lmdb::Database>>(&'b self,
                                                         txn: &'b T,
                                                         path: &Path)
                                                         -> Result<DirEntry> {
        path.components.iter().fold(Ok(DirEntry::root()), |parent_direntry, component| {
            self.find_child(txn, try!(parent_direntry).id, component)
        })
    }

    fn find_entry<'b, T: Transaction<lmdb::Database>>(&'b self,
                                                      txn: &'b T,
                                                      path: &Path)
                                                      -> Result<Entry> {
        let direntry = try!(self.find_direntry(txn, path));

        let entry_bytes = try!(txn.get(self.entries, &direntry.id.as_bytes())
            .map_err(|_| Error::DbCorrupt));

        let objectclass = try!(ObjectClass::from_bytes(&entry_bytes).map_err(|_| Error::DbCorrupt));

        Ok(Entry {
            direntry: direntry,
            objectclass: objectclass,
        })
    }
}

// TODO: since LMDB is ordered, we could e.g. perform a binary search for find
macro_rules! impl_transaction (($newtype:ident) => (
    impl<'a> Transaction<lmdb::Database> for $newtype<'a> {
        fn get(&self, database: Database, key: &[u8]) -> Result<&[u8]> {
            self.0.get(database, &key).map_err(|_| Error::NotFound)
        }

        fn find<P>(&self, db: Database, key: &[u8], predicate: P) -> Result<&[u8]>
            where P: Fn(&[u8]) -> bool
        {
            let mut cursor = try!(self.0
                                      .open_ro_cursor(db)
                                      .map_err(|_| Error::Transaction));

            let mut result = None;

            for (cursor_key, value) in cursor.iter_from(key) {
                if cursor_key != key {
                    return Err(Error::NotFound);
                }

                if predicate(value) {
                    result = Some(value);
                    break;
                }
            }

            result.ok_or(Error::NotFound)
        }

        fn commit(self) -> Result<()> {
            self.0.commit().map_err(|_| Error::Transaction)
        }
    }
));

impl_transaction!(RwTransaction);
impl_transaction!(RoTransaction);

impl<'a> RwTransaction<'a> {
    fn put(&mut self, database: Database, key: &[u8], data: &[u8]) -> Result<()> {
        self.0
            .put(database, &key, &data, WriteFlags::empty())
            .map_err(|_| Error::Transaction)
    }
}

#[cfg(test)]
mod tests {
    use adapter::{Adapter, Transaction};
    use block;
    use error::Error;
    use id::Id;
    use lmdb_adapter::LmdbAdapter;
    use objectclass::ObjectClass;
    use path::Path;

    use lmdb_adapter::tempdir::TempDir;

    fn create_database() -> LmdbAdapter {
        let dir = TempDir::new("ithos-test").unwrap();
        LmdbAdapter::create_database(dir.path()).unwrap()
    }

    #[test]
    fn test_duplicate_block() {
        let adapter = create_database();
        let block = block::tests::example_block();

        let mut txn = adapter.rw_transaction().unwrap();
        adapter.add_block(&mut txn, &block).unwrap();
        txn.commit().unwrap();

        let mut txn = adapter.rw_transaction().unwrap();
        let result = adapter.add_block(&mut txn, &block);
        assert_eq!(result, Err(Error::EntryAlreadyExists));
    }

    #[test]
    fn test_entry_lookup() {
        let adapter = create_database();

        {
            let mut txn = adapter.rw_transaction().unwrap();

            let domain_id = adapter.next_available_id(&txn).unwrap();
            adapter.add_entry(&mut txn,
                           domain_id,
                           Id::root(),
                           "example.com",
                           ObjectClass::Domain)
                .unwrap();

            let hosts_id = domain_id.next();
            adapter.add_entry(&mut txn, hosts_id, domain_id, "hosts", ObjectClass::Ou).unwrap();

            let host_id = hosts_id.next();
            adapter.add_entry(&mut txn,
                           host_id,
                           hosts_id,
                           "master.example.com",
                           ObjectClass::Host)
                .unwrap();

            txn.commit().unwrap();
        }

        {
            let txn = adapter.ro_transaction().unwrap();

            {
                let path = Path::new("/example.com/hosts/master.example.com").unwrap();
                let entry = adapter.find_entry(&txn, &path)
                    .unwrap();

                assert_eq!(entry.direntry.name, "master.example.com");
                assert_eq!(entry.objectclass, ObjectClass::Host);
            }

            txn.commit().unwrap();
        }
    }

    #[test]
    fn test_duplicate_entry_id() {
        let adapter = create_database();

        let mut txn = adapter.rw_transaction().unwrap();

        let domain_id = adapter.next_available_id(&txn).unwrap();
        adapter.add_entry(&mut txn,
                       domain_id,
                       Id::root(),
                       "example.com",
                       ObjectClass::Domain)
            .unwrap();

        let result = adapter.add_entry(&mut txn,
                                       domain_id,
                                       Id::root(),
                                       "another.com",
                                       ObjectClass::Domain);
        assert_eq!(result, Err(Error::EntryAlreadyExists));
    }

    #[test]
    fn test_duplicate_entry_name() {
        let adapter = create_database();

        let mut txn = adapter.rw_transaction().unwrap();

        let domain_id = adapter.next_available_id(&txn).unwrap();
        adapter.add_entry(&mut txn,
                       domain_id,
                       Id::root(),
                       "example.com",
                       ObjectClass::Domain)
            .unwrap();

        let result = adapter.add_entry(&mut txn,
                                       domain_id.next(),
                                       Id::root(),
                                       "example.com",
                                       ObjectClass::Domain);

        assert_eq!(result, Err(Error::EntryAlreadyExists));
    }
}
