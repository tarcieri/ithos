//! setup.rs: A bunch of hastily written code for bootstrapping a new log
//!
//! This bootstraps the minimum viable directory structure to permit additional (authenticated)
//! operations. It creates the following directory hierarchy:
//!
//! 1) Root object (ala LDAP root DSE)
//! 2) Global domain
//! 3) Root administrator account ("manager")
//! 4) Signing key for root administrator account
//!
//! Once this has been done, all subsequent changes to the log can be authorized by credentials
//!

use alg::{CipherSuite, EncryptionAlg, DigestAlg};
use block::{Block, Body};
use crypto::signing::KeyPair;
use id::BlockId;
use object::Object;
use object::credential::{self, Credential};
use object::domain::Domain;
use object::org_unit::OrgUnit;
use object::root::Root;
use object::system::System;
use op::{self, Op};
use path::PathBuf;
use protobuf::RepeatedField;
use timestamp::Timestamp;

const ADMIN_KEYPAIR_LIFETIME: u64 = 315_532_800; // 10 years

// Create the first block in a new log, with a parent ID of zero.
//
// This block contains the initial administrative signature key which will
// be used as the initial root authority for new blocks in the log.
//
// The block is self-signed with the initial administrator key.
//
// TODO: Refactor this mess or create a policy language for these tasks instead
pub fn create_log(ciphersuite: CipherSuite,
                  admin_username: &str,
                  admin_keypair: &KeyPair,
                  admin_keypair_sealed: &[u8],
                  admin_keypair_salt: &[u8],
                  comment: &str)
                  -> Block {
    // This is the only ciphersuite we presently support
    assert!(ciphersuite == CipherSuite::Ed25519_AES256GCM_SHA256);

    let timestamp = Timestamp::now();

    let mut ops = Vec::new();
    let mut path = PathBuf::new();

    // Root DSE
    let mut root = Root::new();
    root.set_digest_alg(DigestAlg::SHA256);

    let mut root_object = Object::new();
    root_object.set_root(root);

    let mut root_op = Op::new();
    root_op.set_optype(op::Type::ADD);
    root_op.set_path(path.clone().into());
    root_op.set_object(root_object);
    ops.push(root_op);

    // Global Domain
    path.push("global");

    let mut global_domain = Domain::new();
    global_domain.set_description(String::from("Global system users and config"));

    let mut global_domain_object = Object::new();
    global_domain_object.set_domain(global_domain);

    let mut global_domain_op = Op::new();
    global_domain_op.set_optype(op::Type::ADD);
    global_domain_op.set_path(path.clone().into());
    global_domain_op.set_object(global_domain_object);
    ops.push(global_domain_op);

    // Users OU
    path.push("users");

    let mut global_users_ou = OrgUnit::new();
    global_users_ou.set_description(String::from("Core system users"));

    let mut global_users_object = Object::new();
    global_users_object.set_org_unit(global_users_ou);

    let mut global_users_op = Op::new();
    global_users_op.set_optype(op::Type::ADD);
    global_users_op.set_path(path.clone().into());
    global_users_op.set_object(global_users_object);
    ops.push(global_users_op);

    // Admin User
    path.push(&admin_username);

    let mut admin_user = System::new();
    admin_user.set_username(String::from(admin_username));

    let mut admin_user_object = Object::new();
    admin_user_object.set_system(admin_user);

    let mut admin_user_op = Op::new();
    admin_user_op.set_optype(op::Type::ADD);
    admin_user_op.set_path(path.clone().into());
    admin_user_op.set_object(admin_user_object);
    ops.push(admin_user_op);

    // Admin Credentials
    path.push("keys");

    let mut admin_keys_ou = OrgUnit::new();
    admin_keys_ou.set_description(String::from("Admin credentials"));

    let mut admin_keys_object = Object::new();
    admin_keys_object.set_org_unit(admin_keys_ou);

    let mut admin_keys_op = Op::new();
    admin_keys_op.set_optype(op::Type::ADD);
    admin_keys_op.set_path(path.clone().into());
    admin_keys_op.set_object(admin_keys_object);
    ops.push(admin_keys_op);

    // Admin Signature Key
    path.push("signing");

    // TODO: honor ciphersuite algorithms
    // TODO: keyid, credential_alg
    let mut admin_signing_credential = Credential::new();
    admin_signing_credential.set_credential_type(credential::Type::SIGNATURE_KEY_PAIR);
    admin_signing_credential.set_sealing_alg(EncryptionAlg::AES256GCM);
    admin_signing_credential.set_encrypted_value(Vec::from(admin_keypair_sealed));
    admin_signing_credential.set_salt(Vec::from(admin_keypair_salt));
    admin_signing_credential.set_public_key(Vec::from(admin_keypair.public_key_bytes()));
    admin_signing_credential.set_not_before(timestamp.to_int());
    admin_signing_credential.set_not_after(timestamp.extend(ADMIN_KEYPAIR_LIFETIME).to_int());
    admin_signing_credential.set_description(String::from("Root signing key"));

    let mut admin_signing_credential_object = Object::new();
    admin_signing_credential_object.set_credential(admin_signing_credential);

    let mut admin_signing_credential_op = Op::new();
    admin_signing_credential_op.set_optype(op::Type::ADD);
    admin_signing_credential_op.set_path(path.into());
    admin_signing_credential_op.set_object(admin_signing_credential_object);
    ops.push(admin_signing_credential_op);

    let mut body = Body::new();
    body.set_parent_id(Vec::from(BlockId::zero().as_ref()));
    body.set_timestamp(timestamp.to_int());
    body.set_ops(RepeatedField::from_vec(ops));
    body.set_comment(comment.to_owned());

    admin_keypair.sign_block(body)
}

#[cfg(test)]
pub mod tests {
    use alg::CipherSuite;
    use crypto::signing::KeyPair;
    use ring::rand;
    use setup;

    pub const ADMIN_USERNAME: &'static str = "manager";
    pub const ADMIN_KEYPAIR_SEALED: &'static [u8] = b"placeholder";
    pub const ADMIN_KEYPAIR_SALT: &'static [u8] = b"NaCl";
    pub const COMMENT: &'static str = "The tree of a thousand users begins with a single block";

    #[test]
    fn test_log_creation() {
        let rng = rand::SystemRandom::new();
        let admin_keypair = KeyPair::generate(&rng);

        setup::create_log(CipherSuite::Ed25519_AES256GCM_SHA256,
                          ADMIN_USERNAME,
                          &admin_keypair,
                          ADMIN_KEYPAIR_SEALED,
                          ADMIN_KEYPAIR_SALT,
                          COMMENT);
    }
}
