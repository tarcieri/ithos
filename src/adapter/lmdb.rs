//! adapter/lmdb.rs: Storage adapter for the Lightning Memory-Mapped Database (LMDB)
//!
//! This adapter is presently the one-and-only storage adapter. Much of the logic it contains
//! should probably be extracted elsewhere before a second storage adapter is written, as a lot of
//! it is reusable and the adapter abstraction is not fully fleshed out.
//!

extern crate lmdb;
extern crate lmdb_sys;

use self::lmdb::{Environment, Database, DatabaseFlags, Cursor, WriteFlags, DUP_SORT, INTEGER_KEY};
use self::lmdb::Error as LmdbError;
use self::lmdb::Transaction as LmdbTransaction;
use adapter::{Adapter, Transaction};
use block::Block;
use direntry::DirEntry;
use entry::SerializedEntry;
use error::{Error, ErrorKind, Result};
use id::{BlockId, EntryId};
use metadata::Metadata;
use path::Path;
use protobuf::{self, Message};
use std::{self, str};
use std::error::Error as StdError;
use std::io::Write;

const MAX_DBS: u32 = 8;
const DB_PERMS: lmdb_sys::mode_t = 0o600;

// Names of "databases" within LMDB: effectively namespaces for keys
const BLOCKS_DB: &'static str = "blocks";
const DIRECTORIES_DB: &'static str = "directories";
const ENTRIES_DB: &'static str = "entries";
const METADATA_DB: &'static str = "metadata";
const STATE_DB: &'static str = "state";

// Names of keys within the "state" database
const LOG_ID_KEY: &'static [u8] = b"log_id";
const LATEST_BLOCK_ID_KEY: &'static [u8] = b"latest_block_id";

pub struct LmdbAdapter {
    env: Environment,
    blocks: Database,
    directories: Database,
    entries: Database,
    metadata: Database,
    state: Database,
}

impl<'a> Adapter<'a> for LmdbAdapter {
    type D = Database;
    type R = RoTransaction<'a>;
    type W = RwTransaction<'a>;

    fn create_database(path: &std::path::Path) -> Result<LmdbAdapter> {
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

    fn open_database(path: &std::path::Path) -> Result<LmdbAdapter> {
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

    fn ro_transaction(&'a self) -> Result<RoTransaction<'a>> {
        match self.env.begin_ro_txn() {
            Ok(txn) => Ok(RoTransaction(txn)),
            Err(_) => Err(Error::transaction(None)),
        }
    }

    fn rw_transaction(&'a self) -> Result<RwTransaction<'a>> {
        match self.env.begin_rw_txn() {
            Ok(txn) => Ok(RwTransaction(txn)),
            Err(_) => Err(Error::transaction(None)),
        }
    }

    fn next_free_entry_id(&self, txn: &RwTransaction) -> Result<EntryId> {
        let cursor = try!(txn.0.open_ro_cursor(self.entries));

        let last_id = match cursor.get(None, None, lmdb_sys::MDB_LAST) {
            Ok((id, _)) => EntryId::from_bytes(id.unwrap()).unwrap(),
            Err(_) => EntryId::root(),
        };

        Ok(last_id.next())
    }

    fn add_block<'t>(&'t self, txn: &'t mut RwTransaction, block: &Block) -> Result<()> {
        let block_id = BlockId::of(block);
        let parent_id = &block.get_body().parent_id;

        // Ensure the block we're adding is the next in the chain
        if *parent_id == BlockId::zero().as_ref() {
            if txn.get(self.state, LOG_ID_KEY) != Err(Error::not_found(None)) {
                return Err(Error::entry_already_exists(Some("initial block already set")));
            }

            try!(txn.put(self.state, LOG_ID_KEY, block_id.as_ref()));
        } else if *parent_id != try!(self.current_block_id(txn)).as_ref() {
            return Err(Error::ordering(Some("new block's parent does not match current ID")));
        }

        // This check should be redundant given the one above, but is here just in case
        if txn.get(self.blocks, block_id.as_ref()) != Err(Error::not_found(None)) {
            return Err(Error::entry_already_exists(Some("new block has already been committed")));
        }

        // Store the new block
        try!(txn.put(self.blocks,
                     block_id.as_ref(),
                     &try!(block.write_to_bytes())));

        // Update the current block ID in the state table
        try!(txn.put(self.state, LATEST_BLOCK_ID_KEY, block_id.as_ref()));

        Ok(())
    }

    fn current_block_id<'t, T>(&'t self, txn: &'t T) -> Result<BlockId>
        where T: Transaction<D = Database>
    {
        BlockId::from_bytes(try!(txn.get(self.state, LATEST_BLOCK_ID_KEY)))
    }

    fn add_entry<'t>(&'t self,
                     txn: &'t mut RwTransaction,
                     entry: &SerializedEntry,
                     name: &'t str,
                     parent_id: EntryId,
                     metadata: &Metadata)
                     -> Result<DirEntry> {
        if txn.get(self.entries, entry.id.as_ref()) != Err(Error::not_found(None)) {
            return Err(Error::entry_already_exists(None));
        }

        if txn.get(self.directories, parent_id.as_ref()) != Err(Error::not_found(None)) &&
           self.find_child(txn, parent_id, name) != Err(Error::not_found(None)) {
            return Err(Error::entry_already_exists(None));
        }

        let direntry = DirEntry {
            id: entry.id,
            parent_id: parent_id,
            name: name,
        };

        if entry.id != EntryId::root() {
            try!(txn.put(self.directories, parent_id.as_ref(), &direntry.to_bytes()));
        }

        try!(txn.put(self.metadata,
                     entry.id.as_ref(),
                     &try!(metadata.write_to_bytes())));

        let mut buffer = try!(txn.reserve(self.entries, entry.id.as_ref(), 4 + entry.data.len()));
        try!(buffer.write_all(&entry.class.as_bytes())
            .map_err(|_| Error::serialize(None)));
        try!(buffer.write_all(entry.data)
            .map_err(|_| Error::serialize(None)));

        Ok(direntry)
    }

    fn find_direntry<'t, T>(&'t self, txn: &'t T, path: &Path) -> Result<DirEntry>
        where T: Transaction<D = Database>
    {
        let result =
            path.components().iter().fold(Ok(DirEntry::root()), |parent_direntry, component| {
                self.find_child(txn, try!(parent_direntry).id, component)
            });

        result.map_err(|e| match e.kind {
            ErrorKind::NotFound => Error::not_found(Some(path.as_ref())),
            _ => e,
        })
    }

    fn find_metadata<'t, T>(&'t self, txn: &'t T, id: &EntryId) -> Result<Metadata>
        where T: Transaction<D = Database>
    {
        let proto = try!(txn.get(self.metadata, id.as_ref()));
        Ok(try!(protobuf::parse_from_bytes::<Metadata>(proto)))
    }

    fn find_entry<'t, T>(&'t self, txn: &'t T, id: &EntryId) -> Result<SerializedEntry>
        where T: Transaction<D = Database>
    {
        let bytes = try!(txn.get(self.entries, id.as_ref()));
        SerializedEntry::from_bytes(*id, bytes)
    }
}

impl LmdbAdapter {
    fn find_child<'a, T>(&'a self, txn: &'a T, parent_id: EntryId, name: &str) -> Result<DirEntry>
        where T: Transaction<D = Database>
    {
        let direntry_bytes = try!(txn.find(self.directories, parent_id.as_ref(), |direntry_bytes| {
                let direntry = DirEntry::new(parent_id, direntry_bytes).unwrap();
                direntry.name == name
            }));

        DirEntry::new(parent_id, direntry_bytes)
    }
}

pub struct RwTransaction<'a>(self::lmdb::RwTransaction<'a>);
pub struct RoTransaction<'a>(self::lmdb::RoTransaction<'a>);

// TODO: since LMDB is ordered, we could e.g. perform a binary search for find
macro_rules! impl_transaction (($newtype:ident) => (
    impl<'a> Transaction for $newtype<'a> {
        type D = Database;

        fn get(&self, db: Database, key: &[u8]) -> Result<&[u8]> {
            self.0.get(db, &key).map_err(|_| Error::not_found(None))
        }

        fn find<P>(&self, db: Database, key: &[u8], predicate: P) -> Result<&[u8]>
            where P: Fn(&[u8]) -> bool
        {
            // Ensure the entry exists
            // TODO: Fix upstream unwrap in lmdb crate's iter_from
            try!(self.get(db, key));

            let mut cursor = try!(self.0.open_ro_cursor(db));
            let mut result = None;

            // TODO: this triggers an unwrap if the key is missing
            for (cursor_key, value) in cursor.iter_from(key) {
                if cursor_key != key {
                    return Err(Error::not_found(None));
                }

                if predicate(value) {
                    result = Some(value);
                    break;
                }
            }

            result.ok_or(Error::not_found(None))
        }

        fn commit(self) -> Result<()> {
            self.0.commit().map_err(|_| Error::transaction(None))
        }
    }
));

impl_transaction!(RwTransaction);
impl_transaction!(RoTransaction);

impl<'a> RwTransaction<'a> {
    pub fn reserve(&mut self, database: Database, key: &[u8], len: usize) -> Result<&mut [u8]> {
        self.0
            .reserve(database, &key, len, WriteFlags::empty())
            .map_err(|_| Error::transaction(None))
    }

    fn put(&mut self, database: Database, key: &[u8], data: &[u8]) -> Result<()> {
        self.0
            .put(database, &key, &data, WriteFlags::empty())
            .map_err(|_| Error::transaction(None))
    }
}

impl From<LmdbError> for Error {
    fn from(error: LmdbError) -> Error {
        match error {
            LmdbError::KeyExist => Error::entry_already_exists(None),
            LmdbError::NotFound => Error::not_found(None),
            _ => {
                let message = format!("{description} (code: {code})",
                                      description = error.description(),
                                      code = error.to_err_code() as i32);

                Error::adapter(Some(&message))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use adapter::{Adapter, Transaction};
    use adapter::lmdb::LmdbAdapter;
    use alg::CipherSuite;
    use block::Block;
    use crypto::signing::KeyPair;
    use entry::{Class, SerializedEntry};
    use error::Error;
    use id::{BlockId, EntryId};
    use metadata::Metadata;
    use path::Path;
    use ring::rand;
    use setup;
    use tempdir::TempDir;
    use timestamp::Timestamp;

    fn create_database() -> LmdbAdapter {
        let dir = TempDir::new("ithos-test").unwrap();
        LmdbAdapter::create_database(dir.path()).unwrap()
    }

    fn example_timestamp() -> Timestamp {
        Timestamp::at(1_231_006_505)
    }

    fn example_block() -> Block {
        let rng = rand::SystemRandom::new();
        let admin_keypair = KeyPair::generate(&rng);

        setup::create_log(CipherSuite::Ed25519_AES256GCM_SHA256,
                          setup::tests::ADMIN_USERNAME,
                          &admin_keypair,
                          setup::tests::ADMIN_KEYPAIR_SEALED,
                          setup::tests::ADMIN_KEYPAIR_SALT,
                          setup::tests::COMMENT)
    }

    fn example_metadata() -> Metadata {
        let mut metadata = Metadata::new();

        let block_id = BlockId::zero();

        metadata.set_created_id(Vec::from(block_id.as_ref()));
        metadata.set_updated_id(Vec::from(block_id.as_ref()));
        metadata.set_created_at(example_timestamp().to_int());
        metadata.set_updated_at(example_timestamp().to_int());

        metadata
    }

    fn example_entry(id: EntryId, data: &[u8]) -> SerializedEntry {
        SerializedEntry {
            id: id,
            class: Class::Root,
            data: data,
        }
    }

    #[test]
    fn duplicate_block() {
        let adapter = create_database();
        let block = example_block();

        let mut txn = adapter.rw_transaction().unwrap();
        adapter.add_block(&mut txn, &block).unwrap();
        txn.commit().unwrap();

        let mut txn = adapter.rw_transaction().unwrap();
        let result = adapter.add_block(&mut txn, &block);
        assert_eq!(result,
                   Err(Error::entry_already_exists(Some("initial block already set"))));
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
                           EntryId::root(),
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

                let direntry = adapter.find_direntry(&txn, path).unwrap();
                assert_eq!(direntry.name, "master.example.com");

                let metadata = adapter.find_metadata(&txn, &direntry.id).unwrap();
                assert_eq!(metadata.created_at, example_timestamp().to_int());

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
                       EntryId::root(),
                       &example_metadata())
            .unwrap();

        let result = adapter.add_entry(&mut txn,
                                       &example_entry(domain_id, b"domain"),
                                       "another.com",
                                       EntryId::root(),
                                       &example_metadata());

        assert_eq!(result, Err(Error::entry_already_exists(None)));
    }

    #[test]
    fn duplicate_entry_name() {
        let adapter = create_database();

        let mut txn = adapter.rw_transaction().unwrap();

        let domain_id = adapter.next_free_entry_id(&txn).unwrap();
        adapter.add_entry(&mut txn,
                       &example_entry(domain_id, b"domain"),
                       "example.com",
                       EntryId::root(),
                       &example_metadata())
            .unwrap();

        let result = adapter.add_entry(&mut txn,
                                       &example_entry(domain_id.next(), b"domain"),
                                       "example.com",
                                       EntryId::root(),
                                       &example_metadata());

        assert_eq!(result, Err(Error::entry_already_exists(None)));
    }
}
