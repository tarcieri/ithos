extern crate lmdb;
extern crate lmdb_sys;

use std::{self, str};
use std::io::Write;

use self::lmdb::{Environment, Database, DatabaseFlags, Cursor, WriteFlags, DUP_SORT, INTEGER_KEY};
use self::lmdb::Error as LmdbError;
use self::lmdb::Transaction as LmdbTransaction;

use adapter::{Adapter, Transaction};
use block::{self, Block};
use direntry::DirEntry;
use entry::{self, Entry};
use error::{Error, Result};
use metadata::Metadata;
use path::Path;
use proto::{FromProto, ToProto};

const MAX_DBS: u32 = 8;
const DB_PERMS: lmdb_sys::mode_t = 0o600;

// Names of "databases" within LMDB: effectively namespaces for keys
const BLOCKS_DB: &'static str = "blocks";
const DIRECTORIES_DB: &'static str = "directories";
const ENTRIES_DB: &'static str = "entries";
const METADATA_DB: &'static str = "metadata";
const STATE_DB: &'static str = "state";

// Names of keys within the "state" database
const LATEST_BLOCK_ID_KEY: &'static [u8] = b"latest_block_id";

pub struct LmdbAdapter {
    env: Environment,
    blocks: Database,
    directories: Database,
    entries: Database,
    metadata: Database,
    state: Database,
}

impl LmdbAdapter {
    pub fn create_database(path: &std::path::Path) -> Result<LmdbAdapter> {
        let env = try!(Environment::new()
            .set_max_dbs(MAX_DBS)
            .open_with_permissions(&path, DB_PERMS));

        let blocks = try!(env.create_db(Some(BLOCKS_DB), DatabaseFlags::empty()));
        let directories = try!(env.create_db(Some(DIRECTORIES_DB), INTEGER_KEY | DUP_SORT));
        let entries = try!(env.create_db(Some(ENTRIES_DB), INTEGER_KEY));
        let metadata = try!(env.create_db(Some(METADATA_DB), INTEGER_KEY));
        let state = try!(env.create_db(Some(STATE_DB), DatabaseFlags::empty()));

        Ok(LmdbAdapter {
            env: env,
            blocks: blocks,
            directories: directories,
            entries: entries,
            metadata: metadata,
            state: state,
        })
    }

    pub fn open_database(path: &std::path::Path) -> Result<LmdbAdapter> {
        let env = try!(Environment::new()
            .set_max_dbs(MAX_DBS)
            .open_with_permissions(&path, DB_PERMS));

        let blocks = try!(env.open_db(Some(BLOCKS_DB)));
        let directories = try!(env.open_db(Some(DIRECTORIES_DB)));
        let entries = try!(env.open_db(Some(ENTRIES_DB)));
        let metadata = try!(env.open_db(Some(METADATA_DB)));
        let state = try!(env.open_db(Some(STATE_DB)));

        Ok(LmdbAdapter {
            env: env,
            blocks: blocks,
            directories: directories,
            entries: entries,
            metadata: metadata,
            state: state,
        })
    }

    fn find_child<'a, T: Transaction<D = Database>>(&'a self,
                                                    txn: &'a T,
                                                    parent_id: entry::Id,
                                                    name: &str)
                                                    -> Result<DirEntry> {
        let direntry_bytes = try!(txn.find(self.directories, parent_id.as_ref(), |direntry_bytes| {
                match DirEntry::new(parent_id, direntry_bytes) {
                    Ok(direntry) => direntry.name == name,
                    _ => false,
                }
            }));

        DirEntry::new(parent_id, direntry_bytes)
    }
}

impl<'a> Adapter<'a> for LmdbAdapter {
    type D = Database;
    type R = RoTransaction<'a>;
    type W = RwTransaction<'a>;

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

    fn next_free_entry_id(&self, txn: &RwTransaction) -> Result<entry::Id> {
        let cursor = try!(txn.0.open_ro_cursor(self.entries));

        let last_id = match cursor.get(None, None, lmdb_sys::MDB_LAST) {
            Ok((id, _)) => entry::Id::from_bytes(id.unwrap()).unwrap(),
            Err(_) => entry::Id::root(),
        };

        Ok(last_id.next())
    }

    fn add_block<'b>(&'b self, txn: &'b mut RwTransaction, block: &Block) -> Result<()> {
        // Ensure the block we're adding is the next in the chain
        if block.parent_id != block::Id::root() &&
           block.parent_id != try!(self.current_block_id(txn)) {
            return Err(Error::Ordering);
        }

        // This check should be redundant given the one above, but is here just in case
        if txn.get(self.blocks, block.id.as_ref()) != Err(Error::NotFound) {
            return Err(Error::EntryAlreadyExists);
        }

        // Store the new block
        try!(txn.put(self.blocks, block.id.as_ref(), &try!(block.to_proto())));

        // Update the current block ID in the state table
        try!(txn.put(self.state, LATEST_BLOCK_ID_KEY, block.id.as_ref()));

        Ok(())
    }

    fn current_block_id<'b, T: Transaction<D = Database>>(&'b self,
                                                          txn: &'b T)
                                                          -> Result<block::Id> {
        block::Id::from_bytes(try!(txn.get(self.state, LATEST_BLOCK_ID_KEY)))
    }

    fn add_entry<'b>(&'b self,
                     txn: &'b mut RwTransaction,
                     entry: &Entry,
                     name: &'b str,
                     parent_id: entry::Id,
                     metadata: &Metadata)
                     -> Result<DirEntry> {
        if txn.get(self.entries, entry.id.as_ref()) != Err(Error::NotFound) {
            return Err(Error::EntryAlreadyExists);
        }

        if txn.get(self.directories, parent_id.as_ref()) != Err(Error::NotFound) &&
           self.find_child(txn, parent_id, name) != Err(Error::NotFound) {
            return Err(Error::EntryAlreadyExists);
        }

        let direntry = DirEntry {
            id: entry.id,
            parent_id: parent_id,
            name: name,
        };

        if entry.id != entry::Id::root() {
            try!(txn.put(self.directories, parent_id.as_ref(), &direntry.to_bytes()));
        }

        try!(txn.put(self.metadata, entry.id.as_ref(), &try!(metadata.to_proto())));

        let mut buffer = try!(txn.reserve(self.entries, entry.id.as_ref(), 4 + entry.data.len()));
        try!(buffer.write_all(&entry.class.as_bytes())
            .map_err(|_| Error::Serialize));
        try!(buffer.write_all(entry.data)
            .map_err(|_| Error::Serialize));

        Ok(direntry)
    }

    fn find_direntry<'b, T: Transaction<D = Database>>(&'b self,
                                                       txn: &'b T,
                                                       path: &Path)
                                                       -> Result<DirEntry> {
        path.components().iter().fold(Ok(DirEntry::root()), |parent_direntry, component| {
            self.find_child(txn, try!(parent_direntry).id, component)
        })
    }

    fn find_metadata<'b, T: Transaction<D = Database>>(&'b self,
                                                       txn: &'b T,
                                                       id: &entry::Id)
                                                       -> Result<Metadata> {
        let proto = try!(txn.get(self.metadata, id.as_ref()));
        Metadata::from_proto(proto)
    }

    fn find_entry<'b, T: Transaction<D = Database>>(&'b self,
                                                    txn: &'b T,
                                                    id: &entry::Id)
                                                    -> Result<Entry> {
        let bytes = try!(txn.get(self.entries, id.as_ref()));
        Entry::from_bytes(*id, bytes)
    }
}

pub struct RwTransaction<'a>(self::lmdb::RwTransaction<'a>);
pub struct RoTransaction<'a>(self::lmdb::RoTransaction<'a>);

// TODO: since LMDB is ordered, we could e.g. perform a binary search for find
macro_rules! impl_transaction (($newtype:ident) => (
    impl<'a> Transaction for $newtype<'a> {
        type D = Database;

        fn get(&self, db: Database, key: &[u8]) -> Result<&[u8]> {
            self.0.get(db, &key).map_err(|_| Error::NotFound)
        }

        fn find<P>(&self, db: Database, key: &[u8], predicate: P) -> Result<&[u8]>
            where P: Fn(&[u8]) -> bool
        {
            let mut cursor = try!(self.0.open_ro_cursor(db));
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
    pub fn reserve(&mut self, database: Database, key: &[u8], len: usize) -> Result<&mut [u8]> {
        self.0
            .reserve(database, &key, len, WriteFlags::empty())
            .map_err(|_| Error::Transaction)
    }

    fn put(&mut self, database: Database, key: &[u8], data: &[u8]) -> Result<()> {
        self.0
            .put(database, &key, &data, WriteFlags::empty())
            .map_err(|_| Error::Transaction)
    }
}

impl From<LmdbError> for Error {
    fn from(error: LmdbError) -> Error {
        match error {
            LmdbError::KeyExist => Error::EntryAlreadyExists,
            LmdbError::NotFound => Error::NotFound,
            _ => Error::Adapter(error.to_err_code() as i32),
        }
    }
}

#[cfg(test)]
mod tests {
    use tempdir::TempDir;

    use adapter::{Adapter, Transaction};
    use adapter::lmdb::LmdbAdapter;
    use block;
    use entry::{Entry, Id};
    use error::Error;
    use metadata::Metadata;
    use object::Class;
    use path::Path;
    use timestamp::Timestamp;

    fn create_database() -> LmdbAdapter {
        let dir = TempDir::new("ithos-test").unwrap();
        LmdbAdapter::create_database(dir.path()).unwrap()
    }

    fn example_timestamp() -> Timestamp {
        Timestamp::at(1_231_006_505)
    }

    fn example_metadata() -> Metadata {
        Metadata::new(block::Id::root(), example_timestamp())
    }

    fn example_entry(id: Id, data: &[u8]) -> Entry {
        Entry {
            id: id,
            class: Class::Root,
            data: data,
        }
    }

    #[test]
    fn duplicate_block() {
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
    fn entry_lookup() {
        let adapter = create_database();
        let example_data = b"just an example host entry";

        {
            let mut txn = adapter.rw_transaction().unwrap();

            let domain_id = adapter.next_free_entry_id(&txn).unwrap();
            adapter.add_entry(&mut txn,
                           &example_entry(domain_id, b"example domain entry"),
                           "example.com",
                           Id::root(),
                           &example_metadata())
                .unwrap();

            let hosts_id = domain_id.next();
            adapter.add_entry(&mut txn,
                           &example_entry(hosts_id, b"example hosts ou"),
                           "hosts",
                           domain_id,
                           &example_metadata())
                .unwrap();

            let host_id = hosts_id.next();
            adapter.add_entry(&mut txn,
                           &example_entry(host_id, example_data),
                           "master.example.com",
                           hosts_id,
                           &example_metadata())
                .unwrap();

            txn.commit().unwrap();
        }

        {
            let txn = adapter.ro_transaction().unwrap();

            {
                let path = Path::new("/example.com/hosts/master.example.com").unwrap();

                let direntry = adapter.find_direntry(&txn, &path).unwrap();
                assert_eq!(direntry.name, "master.example.com");

                let metadata = adapter.find_metadata(&txn, &direntry.id).unwrap();
                assert_eq!(metadata.created_at, example_timestamp());

                let entry = adapter.find_entry(&txn, &direntry.id).unwrap();
                assert_eq!(entry.data, &example_data[..]);
            }

            txn.commit().unwrap();
        }
    }

    #[test]
    fn duplicate_entry_id() {
        let adapter = create_database();

        let mut txn = adapter.rw_transaction().unwrap();

        let domain_id = adapter.next_free_entry_id(&txn).unwrap();
        adapter.add_entry(&mut txn,
                       &example_entry(domain_id, b"domain"),
                       "example.com",
                       Id::root(),
                       &example_metadata())
            .unwrap();

        let result = adapter.add_entry(&mut txn,
                                       &example_entry(domain_id, b"domain"),
                                       "another.com",
                                       Id::root(),
                                       &example_metadata());

        assert_eq!(result, Err(Error::EntryAlreadyExists));
    }

    #[test]
    fn duplicate_entry_name() {
        let adapter = create_database();

        let mut txn = adapter.rw_transaction().unwrap();

        let domain_id = adapter.next_free_entry_id(&txn).unwrap();
        adapter.add_entry(&mut txn,
                       &example_entry(domain_id, b"domain"),
                       "example.com",
                       Id::root(),
                       &example_metadata())
            .unwrap();

        let result = adapter.add_entry(&mut txn,
                                       &example_entry(domain_id.next(), b"domain"),
                                       "example.com",
                                       Id::root(),
                                       &example_metadata());

        assert_eq!(result, Err(Error::EntryAlreadyExists));
    }
}
