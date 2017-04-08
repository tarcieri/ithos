//! adapter/lmdb.rs: Storage adapter for the Lightning Memory-Mapped Database (LMDB)
//!
//! This adapter is presently the one-and-only storage adapter. Much of the logic it contains
//! should probably be extracted elsewhere before a second storage adapter is written, as a lot of
//! it is reusable and the adapter abstraction is not fully fleshed out.
//!

extern crate lmdb;
extern crate lmdb_sys;

use self::lmdb::{Environment, Database, DatabaseFlags, Cursor, WriteFlags, DUP_SORT, INTEGER_KEY};
pub use self::lmdb::Error as LmdbError;
use self::lmdb::Transaction as LmdbTransaction;
use adapter::{Adapter, Transaction};
use block::Block;
use direntry::DirEntry;
use entry::{self, SerializedEntry};
use errors::*;
use id::{BlockId, EntryId};
use metadata::Metadata;
use path::Path;
use protobuf::{self, Message};
use std::io::Write;
use std::path::Path as StdPath;
use std::str;

const MAX_DBS: u32 = 8;
const DB_PERMS: lmdb_sys::mode_t = 0o600;

// Names of "databases" within LMDB: effectively namespaces for keys
const BLOCKS_DB: &str = "blocks";
const DIRECTORIES_DB: &str = "directories";
const ENTRIES_DB: &str = "entries";
const METADATA_DB: &str = "metadata";
const STATE_DB: &str = "state";

// Names of keys within the "state" database
const LOG_ID_KEY: &[u8] = b"log_id";
const LATEST_BLOCK_ID_KEY: &[u8] = b"latest_block_id";

/// Adapter implementation for the Lightning Memory Database
pub struct LmdbAdapter {
    /// LMDB "environment" (directory containing multiple databases)
    env: Environment,

    /// Blocks in the log, persisted by ID
    blocks: Database,

    /// Directory hierarchy, mapping names to entry IDs
    directories: Database,

    /// Entries indexed by integer entry ID
    entries: Database,

    /// Per-entry metadata, e.g. creation/modification time
    metadata: Database,

    /// Global metadata about the current state of the directory
    state: Database,
}

impl<'a> Adapter<'a> for LmdbAdapter {
    type R = RoTransaction<'a>;
    type W = RwTransaction<'a>;

    fn create_database(path: &StdPath) -> Result<LmdbAdapter> {
        let env = Environment::new().set_max_dbs(MAX_DBS)
            .open_with_permissions(path, DB_PERMS)?;

        let blocks = env.create_db(Some(BLOCKS_DB), DatabaseFlags::empty())?;
        let directories = env.create_db(Some(DIRECTORIES_DB), INTEGER_KEY | DUP_SORT)?;
        let entries = env.create_db(Some(ENTRIES_DB), INTEGER_KEY)?;
        let metadata = env.create_db(Some(METADATA_DB), INTEGER_KEY)?;
        let state = env.create_db(Some(STATE_DB), DatabaseFlags::empty())?;

        Ok(LmdbAdapter {
            env: env,
            blocks: blocks,
            directories: directories,
            entries: entries,
            metadata: metadata,
            state: state,
        })
    }

    fn open_database(path: &StdPath) -> Result<LmdbAdapter> {
        let env = Environment::new().set_max_dbs(MAX_DBS)
            .open_with_permissions(path, DB_PERMS)?;

        let blocks = env.open_db(Some(BLOCKS_DB))?;
        let directories = env.open_db(Some(DIRECTORIES_DB))?;
        let entries = env.open_db(Some(ENTRIES_DB))?;
        let metadata = env.open_db(Some(METADATA_DB))?;
        let state = env.open_db(Some(STATE_DB))?;

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
        Ok(RoTransaction(self.env.begin_ro_txn()?))
    }

    fn rw_transaction(&'a self) -> Result<RwTransaction<'a>> {
        Ok(RwTransaction(self.env.begin_rw_txn()?))
    }

    fn next_free_entry_id(&self, txn: &RwTransaction) -> Result<EntryId> {
        let cursor = txn.0.open_ro_cursor(self.entries)?;

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
            match txn.lmdb_get(self.state, LOG_ID_KEY) {
                Ok(_) => {
                    let msg = "initial block already set".to_string();
                    return Err(ErrorKind::EntryAlreadyExists(msg).into());
                }
                Err(Error(ErrorKind::Lmdb(LmdbError::NotFound), _)) => (),
                Err(err) => return Err(err),
            };

            txn.put(self.state, LOG_ID_KEY, block_id.as_ref())?;
        } else if *parent_id != self.current_block_id(txn)?.as_ref() {
            let msg = "new block's parent does not match current ID".to_string();
            return Err(ErrorKind::OrderingInvalid(msg).into());
        }

        // This check should be redundant given the one above, but is here just in case
        match txn.lmdb_get(self.blocks, block_id.as_ref()) {
            Ok(_) => {
                let msg = "new block has already been committed".to_string();
                return Err(ErrorKind::EntryAlreadyExists(msg).into());
            }
            Err(Error(ErrorKind::Lmdb(LmdbError::NotFound), _)) => (),
            Err(err) => return Err(err),
        }

        // Store the new block
        txn.put(self.blocks, block_id.as_ref(), &block.write_to_bytes()?)?;

        // Update the current block ID in the state table
        txn.put(self.state, LATEST_BLOCK_ID_KEY, block_id.as_ref())?;

        Ok(())
    }

    fn current_block_id<'t, T>(&'t self, txn: &'t T) -> Result<BlockId>
        where T: Transaction
    {
        BlockId::from_bytes(txn.lmdb_get(self.state, LATEST_BLOCK_ID_KEY)?)
    }

    fn add_entry<'t>(&'t self,
                     txn: &'t mut RwTransaction,
                     entry: &SerializedEntry,
                     name: &'t str,
                     parent_id: EntryId,
                     metadata: &Metadata)
                     -> Result<DirEntry> {
        match txn.lmdb_get(self.entries, entry.id.as_ref()) {
            Ok(_) => {
                let msg = format!("error creating '{}': entry ID '{:?}' already exists",
                                  name,
                                  entry.id);
                return Err(ErrorKind::EntryAlreadyExists(msg).into());
            }
            Err(Error(ErrorKind::Lmdb(LmdbError::NotFound), _)) => (),
            Err(err) => return Err(err).chain_err(|| format!("error creating {}", name)),
        }

        match txn.lmdb_get(self.directories, parent_id.as_ref()) {
            Ok(_) => {
                match self.find_child(txn, parent_id, name) {
                    Ok(_) => {
                        let msg = format!("entry '{}' already exists", name);
                        return Err(ErrorKind::EntryAlreadyExists(msg).into());
                    }
                    Err(Error(ErrorKind::NotFound(_), _)) => (),
                    Err(err) => {
                        return Err(err).chain_err(|| {
                            format!("error finding child of entry ID {:?}", parent_id)
                        })
                    }
                }
            }
            Err(Error(ErrorKind::Lmdb(LmdbError::NotFound), _)) => (),
            Err(err) => return Err(err).chain_err(|| format!("error creating {}", name)),
        }

        let direntry = DirEntry {
            id: entry.id,
            parent_id: parent_id,
            name: name,
        };

        if entry.id != EntryId::root() {
            txn.put(self.directories, parent_id.as_ref(), &direntry.to_bytes())?;
        }

        txn.put(self.metadata,
                 entry.id.as_ref(),
                 &metadata.write_to_bytes()?)?;

        let mut buffer = txn.reserve(self.entries,
                     entry.id.as_ref(),
                     entry::HEADER_SIZE + entry.data.len())?;

        buffer.write_all(&entry.class.as_bytes())?;
        buffer.write_all(entry.data)?;

        Ok(direntry)
    }

    fn find_direntry<'t, T>(&'t self, txn: &'t T, path: &Path) -> Result<DirEntry>
        where T: Transaction
    {
        let result =
            path.components().iter().fold(Ok(DirEntry::root()), |parent_direntry, component| {
                self.find_child(txn, parent_direntry?.id, component)
            });

        match result {
            Ok(_) => result,
            Err(Error(ErrorKind::Lmdb(LmdbError::NotFound), _)) => {
                result.chain_err(|| format!("not found: {:?}", path))
            }
            Err(_) => result.chain_err(|| format!("error reading {:?}", path)),
        }
    }

    fn find_metadata<'t, T>(&'t self, txn: &'t T, id: &EntryId) -> Result<Metadata>
        where T: Transaction
    {
        let proto = txn.lmdb_get(self.metadata, id.as_ref())?;
        Ok(protobuf::parse_from_bytes::<Metadata>(proto)?)
    }

    fn find_entry<'t, T>(&'t self, txn: &'t T, id: &EntryId) -> Result<SerializedEntry>
        where T: Transaction
    {
        let bytes = txn.lmdb_get(self.entries, id.as_ref())?;
        SerializedEntry::from_bytes(*id, bytes)
    }
}

impl LmdbAdapter {
    fn find_child<'a, T>(&'a self, txn: &'a T, parent_id: EntryId, name: &str) -> Result<DirEntry>
        where T: Transaction
    {
        let direntry_bytes = txn.lmdb_find(self.directories, parent_id.as_ref(), |direntry_bytes| {
                let direntry = DirEntry::new(parent_id, direntry_bytes).unwrap();
                direntry.name == name
            })?;

        DirEntry::new(parent_id, direntry_bytes)
    }
}

/// Internal functionality which is not part of the public `Transaction` API
/// NOTE: these methods should not be called from outside of this module
pub trait AdapterTransaction {
    /// Underlying transaction type from the `lmdb` crate
    type T: LmdbTransaction;

    /// Obtain the inner `LmdbTransaction`
    fn lmdb_txn(&self) -> &Self::T;

    /// Get the raw data associated with an object (TODO: remove this from this trait)
    fn lmdb_get(&self, db: Database, key: &[u8]) -> Result<&[u8]> {
        Ok(self.lmdb_txn().get(db, &key)?)
    }

    /// Perform a search of the given database, looking for an entry that matches the predicate
    // TODO: since LMDB is ordered, we could e.g. perform a binary search
    fn lmdb_find<P>(&self, db: Database, key: &[u8], predicate: P) -> Result<&[u8]>
        where P: Fn(&[u8]) -> bool
    {
        // Ensure the entry exists
        // TODO: Fix upstream unwrap in lmdb crate's iter_from
        self.lmdb_get(db, key)?;

        let mut cursor = self.lmdb_txn().open_ro_cursor(db)?;
        let mut result = None;

        // TODO: Remove earlier check once this no longer panics on missing keys
        for (cursor_key, value) in cursor.iter_from(key) {
            if cursor_key != key {
                return Err(ErrorKind::NotFound("key not found".to_string()).into());
            }

            if predicate(value) {
                result = Some(value);
                break;
            }
        }

        result.ok_or_else(|| ErrorKind::NotFound("key not found".to_string()).into())
    }
}

/// Read-write transaction: only one allowed at a time
pub struct RwTransaction<'a>(self::lmdb::RwTransaction<'a>);

/// Read-only transaction: several can be active concurrently
pub struct RoTransaction<'a>(self::lmdb::RoTransaction<'a>);

impl<'a> AdapterTransaction for RwTransaction<'a> {
    type T = self::lmdb::RwTransaction<'a>;

    fn lmdb_txn(&self) -> &self::lmdb::RwTransaction<'a> {
        &self.0
    }
}

impl<'a> AdapterTransaction for RoTransaction<'a> {
    type T = self::lmdb::RoTransaction<'a>;

    fn lmdb_txn(&self) -> &self::lmdb::RoTransaction<'a> {
        &self.0
    }
}

impl<'a> Transaction for RwTransaction<'a> {
    fn commit(self) -> Result<()> {
        Ok(self.0.commit()?)
    }
}

impl<'a> Transaction for RoTransaction<'a> {
    fn commit(self) -> Result<()> {
        Ok(self.0.commit()?)
    }
}

impl<'a> RwTransaction<'a> {
    /// Reserve the given amount of space in LMDB for the given key
    pub fn reserve(&mut self, database: Database, key: &[u8], len: usize) -> Result<&mut [u8]> {
        Ok(self.0.reserve(database, &key, len, WriteFlags::empty())?)

    }

    /// Put the given data into LMDB under the given key
    fn put(&mut self, database: Database, key: &[u8], data: &[u8]) -> Result<()> {
        Ok(self.0.put(database, &key, &data, WriteFlags::empty())?)
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
    use errors::*;
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
        let err = adapter.add_block(&mut txn, &block)
            .expect_err("expected duplicate block to cause error");

        match *err.kind() {
            ErrorKind::EntryAlreadyExists(ref msg) => assert_eq!(msg, "initial block already set"),
            ref other => panic!("unexpected error kind: {:?}", other),
        }
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

        let err = adapter.add_entry(&mut txn,
                       &example_entry(domain_id, b"domain"),
                       "another.com",
                       EntryId::root(),
                       &example_metadata())
            .expect_err("expected duplicate entry ID to cause error");

        match *err.kind() {
            ErrorKind::EntryAlreadyExists(_) => (),
            ref other => panic!("unexpected error kind: {:?}", other),
        }
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

        let err = adapter.add_entry(&mut txn,
                       &example_entry(domain_id.next(), b"domain"),
                       "example.com",
                       EntryId::root(),
                       &example_metadata())
            .expect_err("expected duplicate entry name to cause error");

        match *err.kind() {
            ErrorKind::EntryAlreadyExists(_) => (),
            ref other => panic!("unexpected error kind: {:?}", other),
        }
    }
}
