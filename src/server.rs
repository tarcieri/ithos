use std::{self, str};

use ring::rand;

use adapter::{Adapter, Transaction};
use adapter::lmdb::LmdbAdapter;
use algorithm::{DigestAlgorithm, SignatureAlgorithm};
use block::Block;
use error::Result;
use log;
use op;
use password::{self, PasswordAlgorithm};
use signature::KeyPair;

#[cfg(test)]
extern crate tempdir;

const DEFAULT_GENESIS_MESSAGE: &'static str = "Initial block";

pub struct Server {
    adapter: LmdbAdapter,
}

impl Server {
    pub fn create_database(path: &std::path::Path,
                           admin_username: &str,
                           admin_password: &str)
                           -> Result<Server> {
        let rng = rand::SystemRandom::new();

        let logid = try!(log::Id::generate(&rng));

        let mut salt = Vec::with_capacity(16 + admin_username.as_bytes().len());
        salt.extend(logid.as_ref());
        salt.extend(admin_username.as_bytes());

        let mut admin_symmetric_key = [0u8; 32];
        password::derive(PasswordAlgorithm::SCRYPT,
                         &salt,
                         &admin_password,
                         &mut admin_symmetric_key);

        let (admin_keypair, admin_keypair_sealed) =
            KeyPair::generate_and_seal(SignatureAlgorithm::Ed25519, &rng, &admin_symmetric_key);

        let genesis_block = Block::genesis_block(&logid,
                                                 &admin_username,
                                                 &admin_keypair,
                                                 &admin_keypair_sealed,
                                                 DEFAULT_GENESIS_MESSAGE,
                                                 DigestAlgorithm::Sha256);

        let adapter = LmdbAdapter::create_database(path).unwrap();
        let server = Server { adapter: adapter };

        try!(server.commit_unverified_block(&genesis_block));
        Ok(server)
    }

    // Commit a block without first checking its signature
    fn commit_unverified_block(&self, block: &Block) -> Result<()> {
        let mut txn = try!(self.adapter.rw_transaction());
        let mut state = op::State::new(try!(self.adapter.next_free_entry_id(&txn)));

        // Process the operations in the block and apply them to the database
        for op in &block.ops {
            try!(op.apply(&self.adapter, &mut txn, &mut state, block));
        }

        // NOTE: This only stores the block in the database. It does not process it
        try!(self.adapter.add_block(&mut txn, block));

        txn.commit()
    }
}

#[cfg(test)]
mod tests {
    use server::Server;
    use server::tempdir::TempDir;

    const ADMIN_USERNAME: &'static str = "manager";
    const ADMIN_PASSWORD: &'static str = "The Magic Words are Squeamish Ossifrage";

    #[test]
    fn test_create_database() {
        let dir = TempDir::new("ithos-test").unwrap();
        Server::create_database(dir.path(), ADMIN_USERNAME, ADMIN_PASSWORD).unwrap();
    }
}
