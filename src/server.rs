use std::{self, str};

use ring::rand::SecureRandom;

use adapter::{Adapter, Transaction};
use algorithm::{DigestAlgorithm, EncryptionAlgorithm, SignatureAlgorithm};
use block::Block;
use encryption::{AES256GCM_KEY_SIZE, AES256GCM_NONCE_SIZE};
use error::{Error, Result};
use log;
use object::Object;
use object::credential::CredentialEntry;
use object::domain::DomainEntry;
use op::{self, Op};
use password::{self, PasswordAlgorithm};
use path::{Path, PathBuf};
use signature::KeyPair;
use timestamp::Timestamp;

#[cfg(test)]
extern crate tempdir;

const DEFAULT_GENESIS_MESSAGE: &'static str = "Initial block";

pub struct Server<A>
    where A: for<'a> Adapter<'a>
{
    adapter: A,
}

impl<A> Server<A>
    where A: for<'a> Adapter<'a>
{
    pub fn create_database(path: &std::path::Path,
                           rng: &SecureRandom,
                           admin_username: &str,
                           admin_password: &str)
                           -> Result<Server<A>> {
        let logid = try!(log::Id::generate(rng));

        let mut salt = Vec::with_capacity(16 + admin_username.as_bytes().len());
        salt.extend(logid.as_ref());
        salt.extend(admin_username.as_bytes());

        let mut admin_symmetric_key = [0u8; AES256GCM_KEY_SIZE];
        password::derive(PasswordAlgorithm::SCRYPT,
                         &salt,
                         &admin_password,
                         &mut admin_symmetric_key);

        let encryption_alg = EncryptionAlgorithm::Aes256Gcm;
        let signature_alg = SignatureAlgorithm::Ed25519;

        // NOTE: Fixed nonce. The admin password should be randomly generated and never reused
        let nonce = [0u8; AES256GCM_NONCE_SIZE];
        let (admin_keypair, admin_keypair_sealed) = try!(KeyPair::generate_and_seal(
                                       signature_alg,
                                       encryption_alg,
                                       rng,
                                       &admin_symmetric_key,
                                       &nonce));

        let genesis_block = Block::genesis_block(&logid,
                                                 &admin_username,
                                                 &admin_keypair,
                                                 &admin_keypair_sealed,
                                                 DEFAULT_GENESIS_MESSAGE,
                                                 DigestAlgorithm::Sha256,
                                                 encryption_alg,
                                                 signature_alg);

        let adapter = try!(A::create_database(path));

        let server = Server { adapter: adapter };
        try!(server.commit_unverified_block(&genesis_block));

        Ok(server)
    }

    pub fn open_database(path: &std::path::Path) -> Result<Server<A>> {
        let adapter = try!(A::open_database(path));
        Ok(Server { adapter: adapter })
    }

    pub fn add_domain(&self,
                      admin_keypair: &KeyPair,
                      domain_name: &str,
                      description: Option<String>,
                      comment: &str)
                      -> Result<()> {
        let domain_entry = DomainEntry { description: description };

        let timestamp = Timestamp::now();
        let mut ops = Vec::new();
        let mut path = PathBuf::new();
        path.push(&domain_name);

        ops.push(Op::new(op::Type::Add, path, Object::Domain(domain_entry)));

        // TODO: make committing the block transactional
        let txn = try!(self.adapter.ro_transaction());
        let parent_id = try!(self.adapter.current_block_id(&txn));
        let block = Block::new(parent_id, timestamp, ops, comment, admin_keypair);

        // TODO: authenticate signature before committing
        try!(self.commit_unverified_block(&block));

        Ok(())
    }

    pub fn find_credential(&self, path: &Path) -> Result<CredentialEntry> {
        let txn = try!(self.adapter.ro_transaction());
        let direntry = try!(self.adapter.find_direntry(&txn, path));
        let entry = try!(self.adapter.find_entry(&txn, &direntry.id));

        match try!(entry.to_object()) {
            Object::Credential(credential_entry) => Ok(credential_entry),
            _ => Err(Error::BadType),
        }
    }

    pub fn find_logid(&self) -> Result<log::Id> {
        let txn = try!(self.adapter.ro_transaction());
        let direntry = try!(self.adapter.find_direntry(&txn, Path::new("/").unwrap()));
        let entry = try!(self.adapter.find_entry(&txn, &direntry.id));

        match try!(entry.to_object()) {
            Object::Root(root_entry) => Ok(root_entry.logid),
            _ => Err(Error::BadType),
        }
    }

    // Commit a block without first checking its signature
    fn commit_unverified_block(&self, block: &Block) -> Result<()> {
        let mut txn = try!(self.adapter.rw_transaction());
        let mut state = op::State::new(try!(self.adapter.next_free_entry_id(&txn)));

        // NOTE: This only stores the block in the database. It does not process it
        try!(self.adapter.add_block(&mut txn, block));

        // Process the operations in the block and apply them to the database
        for op in &block.ops {
            try!(op.apply(&self.adapter,
                          &mut txn,
                          &mut state,
                          &block.id,
                          block.timestamp));
        }

        txn.commit()
    }
}

#[cfg(test)]
mod tests {
    use ring::rand;

    use adapter::lmdb::LmdbAdapter;
    use encryption::AES256GCM_KEY_SIZE;
    use path::PathBuf;
    use password::{self, PasswordAlgorithm};
    use server::Server;
    use server::tempdir::TempDir;
    use signature;

    const ADMIN_USERNAME: &'static str = "manager";
    const ADMIN_PASSWORD: &'static str = "The Magic Words are Squeamish Ossifrage";
    const EXAMPLE_DOMAIN: &'static str = "example.com";

    fn create_database() -> Server<LmdbAdapter> {
        let rng = rand::SystemRandom::new();
        let dir = TempDir::new("ithos-test").unwrap();
        Server::create_database(dir.path(), &rng, ADMIN_USERNAME, ADMIN_PASSWORD).unwrap()
    }

    fn admin_keypair(server: &Server<LmdbAdapter>) -> signature::KeyPair {
        let mut keypair_path = PathBuf::new();
        keypair_path.push("system");
        keypair_path.push(ADMIN_USERNAME);
        keypair_path.push("keys");
        keypair_path.push("signing");

        let credential = server.find_credential(keypair_path.as_ref()).unwrap();
        let logid = server.find_logid().unwrap();

        let mut salt = Vec::with_capacity(16 + ADMIN_USERNAME.as_bytes().len());
        salt.extend(logid.as_ref());
        salt.extend(ADMIN_USERNAME.as_bytes());

        let mut admin_symmetric_key = [0u8; AES256GCM_KEY_SIZE];
        password::derive(PasswordAlgorithm::SCRYPT,
                         &salt,
                         ADMIN_PASSWORD,
                         &mut admin_symmetric_key);

        credential.unseal_signature_keypair(&admin_symmetric_key).unwrap()
    }

    #[test]
    fn test_add_domain() {
        let server = create_database();
        let keypair = admin_keypair(&server);

        server.add_domain(&keypair, EXAMPLE_DOMAIN, None, "Testing 1 2 3").unwrap();
    }
}
