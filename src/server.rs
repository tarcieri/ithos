//! server.rs: The core ithos daemon and related admin functionality
//!
//! This is presently a bit of a dumping ground for server-side functionality
//!
//! Logic in here shouldn't get too complicated. This is (accidentally) prevented in part by
//! the limitations of Rust's type system, as higher ranked trait bounds on Adapter prevent
//! access to the associated types for transactions.
//!

use adapter::Adapter;
use algorithm::{CipherSuite, SignatureAlgorithm, EncryptionAlgorithm};
use block::Body;
use crypto::signing::KeyPair;
use crypto::symmetric::{AES256GCM_KEY_SIZE, AES256GCM_NONCE_SIZE};
use entry::Entry;
use error::{Error, Result};
use object::Object;
use object::credential::Credential;
use object::domain::Domain;
use op::{self, Op};
use password::{self, PasswordAlgorithm};
use path::{Path, PathBuf};
use protobuf::RepeatedField;
use ring::rand::SecureRandom;
use setup;
use std::{self, str};
use timestamp::Timestamp;
use transform::Transform;

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
                           ciphersuite: CipherSuite,
                           admin_username: &str,
                           admin_password: &str)
                           -> Result<()> {
        // We presently only support one ciphersuite
        assert!(ciphersuite == CipherSuite::Ed25519_AES256GCM_SHA256);

        let admin_keypair_salt = try!(password::random_salt(rng));

        let mut admin_symmetric_key = [0u8; AES256GCM_KEY_SIZE];
        password::derive(PasswordAlgorithm::SCRYPT,
                         &admin_keypair_salt,
                         admin_password,
                         &mut admin_symmetric_key);

        // NOTE: Fixed nonce. The admin password should be randomly generated and never reused
        let nonce = [0u8; AES256GCM_NONCE_SIZE];

        // TODO: honor ciphersuite algorithms
        let (admin_keypair, admin_keypair_sealed) = try!(KeyPair::generate_and_seal(
                                       SignatureAlgorithm::Ed25519,
                                       EncryptionAlgorithm::AES256GCM,
                                       rng,
                                       &admin_symmetric_key,
                                       &nonce));

        let initial_block = setup::create_log(ciphersuite,
                                              admin_username,
                                              &admin_keypair,
                                              &admin_keypair_sealed,
                                              &admin_keypair_salt,
                                              DEFAULT_INITIAL_BLOCK_COMMENT);

        let adapter = try!(A::create_database(path));

        let mut transform = try!(Transform::new(&adapter));
        try!(transform.apply(&initial_block));
        try!(transform.commit());

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
        let mut domain_entry = Domain::new();

        if let Some(desc) = description {
            domain_entry.set_description(desc);
        }

        let timestamp = Timestamp::now();
        let mut path = PathBuf::new();
        path.push(&domain_name);

        let mut domain_entry_object = Object::new();
        domain_entry_object.set_domain(domain_entry);

        let mut op = Op::new();
        op.set_optype(op::Type::ADD);
        op.set_path(path.into());
        op.set_object(domain_entry_object);

        let mut transform = try!(Transform::new(&self.adapter));

        let mut body = Body::new();
        body.set_parent_id(Vec::from(try!(transform.block_id()).as_ref()));
        body.set_timestamp(timestamp.to_int());
        body.set_ops(RepeatedField::from_vec(vec![op]));
        body.set_comment(comment.to_owned());

        let block = admin_keypair.sign_block(body);

        // TODO: authenticate signature before committing (BIG SECURITY PROBLEM!!!)
        try!(transform.apply(&block));
        try!(transform.commit());

        Ok(())
    }

    pub fn find_credential(&self, path: &Path) -> Result<Credential> {
        match try!(Entry::find(&self.adapter, path)) {
            Entry::Credential(credential_entry) => Ok(credential_entry),
            _ => Err(Error::bad_type(None)),
        }
    }
}

#[cfg(test)]
mod tests {
    use adapter::lmdb::LmdbAdapter;
    use algorithm::CipherSuite;
    use crypto::signing::KeyPair;
    use crypto::symmetric::AES256GCM_KEY_SIZE;
    use password::{self, PasswordAlgorithm};
    use path::PathBuf;
    use ring::rand;
    use server::Server;
    use server::tempdir::TempDir;

    const ADMIN_USERNAME: &'static str = "manager";
    const ADMIN_PASSWORD: &'static str = "The Magic Words are Squeamish Ossifrage";
    const EXAMPLE_DOMAIN: &'static str = "example.com";

    fn create_database() -> Server<LmdbAdapter> {
        let rng = rand::SystemRandom::new();
        let dir = TempDir::new("ithos-test").unwrap();
        Server::<LmdbAdapter>::create_database(dir.path(),
                                               &rng,
                                               CipherSuite::Ed25519_AES256GCM_SHA256,
                                               ADMIN_USERNAME,
                                               ADMIN_PASSWORD)
            .unwrap();
        Server::<LmdbAdapter>::open_database(dir.path()).unwrap()
    }

    fn admin_keypair(server: &Server<LmdbAdapter>) -> KeyPair {
        let mut keypair_path = PathBuf::new();
        keypair_path.push("global");
        keypair_path.push("users");
        keypair_path.push(ADMIN_USERNAME);
        keypair_path.push("keys");
        keypair_path.push("signing");

        let credential = server.find_credential(keypair_path.as_ref()).unwrap();

        let mut admin_symmetric_key = [0u8; AES256GCM_KEY_SIZE];

        password::derive(PasswordAlgorithm::SCRYPT,
                         &credential.salt,
                         ADMIN_PASSWORD,
                         &mut admin_symmetric_key);

        KeyPair::unseal_from_credential(&credential, &admin_symmetric_key).unwrap()
    }

    #[test]
    fn test_add_domain() {
        let server = create_database();
        let keypair = admin_keypair(&server);

        server.add_domain(&keypair, EXAMPLE_DOMAIN, None, "Testing 1 2 3").unwrap();
    }
}
