use std::{self, str};

use ring::rand::SecureRandom;

use adapter::{Adapter, Transaction};
use algorithm::{DigestAlgorithm, EncryptionAlgorithm, SignatureAlgorithm};
use block::Block;
use encryption::{AES256GCM_KEY_SIZE, AES256GCM_NONCE_SIZE};
use error::{Error, Result};
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

// Comment
const DEFAULT_INITIAL_BLOCK_COMMENT: &'static str = "Initial log creation";

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
                           -> Result<()> {
        let admin_keypair_salt = try!(password::random_salt(rng));

        let mut admin_symmetric_key = [0u8; AES256GCM_KEY_SIZE];
        password::derive(PasswordAlgorithm::SCRYPT,
                         &admin_keypair_salt,
                         admin_password,
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

        let initial_block = Block::initial_block(admin_username,
                                                 &admin_keypair,
                                                 &admin_keypair_sealed,
                                                 &admin_keypair_salt,
                                                 DEFAULT_INITIAL_BLOCK_COMMENT,
                                                 DigestAlgorithm::Sha256,
                                                 encryption_alg,
                                                 signature_alg);

        let adapter = try!(A::create_database(path));
        let mut txn = try!(adapter.rw_transaction());

        try!(initial_block.apply(&adapter, &mut txn));
        try!(txn.commit());

        Ok(())
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
        let domain_entry = DomainEntry::new(description);

        let timestamp = Timestamp::now();
        let mut ops = Vec::new();
        let mut path = PathBuf::new();
        path.push(&domain_name);

        ops.push(Op::new(op::Type::Add, path, Object::Domain(domain_entry)));

        let mut txn = try!(self.adapter.rw_transaction());
        let parent_id = try!(self.adapter.current_block_id(&txn));
        let block = Block::new(parent_id, timestamp, ops, comment, admin_keypair);

        // TODO: authenticate signature before committing
        try!(block.apply(&self.adapter, &mut txn));
        try!(txn.commit());

        Ok(())
    }

    pub fn find_credential(&self, path: &Path) -> Result<CredentialEntry> {
        match try!(Object::find(&self.adapter, path)) {
            Object::Credential(credential_entry) => Ok(credential_entry),
            _ => Err(Error::BadType),
        }
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
        Server::<LmdbAdapter>::create_database(dir.path(), &rng, ADMIN_USERNAME, ADMIN_PASSWORD)
            .unwrap();
        Server::<LmdbAdapter>::open_database(dir.path()).unwrap()
    }

    fn admin_keypair(server: &Server<LmdbAdapter>) -> signature::KeyPair {
        let mut keypair_path = PathBuf::new();
        keypair_path.push("global");
        keypair_path.push("users");
        keypair_path.push(ADMIN_USERNAME);
        keypair_path.push("keys");
        keypair_path.push("signing");

        let credential = server.find_credential(keypair_path.as_ref()).unwrap();

        let salt = match credential.salt {
            Some(ref s) => s,
            None => panic!("salt missing!"),
        };

        let mut admin_symmetric_key = [0u8; AES256GCM_KEY_SIZE];
        password::derive(PasswordAlgorithm::SCRYPT,
                         salt,
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
