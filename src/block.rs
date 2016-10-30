use std::io;

use buffoon::{OutputStream, Serialize};
use rustc_serialize::base64::{self, ToBase64};
use serde_json;
use serde_json::builder::ObjectBuilder;

use algorithm::{DigestAlgorithm, EncryptionAlgorithm, SignatureAlgorithm};
use error::{Error, Result};
use log;
use object::Object;
use object::credential::CredentialEntry;
use object::ou::OrgUnitEntry;
use object::root::RootEntry;
use object::system::SystemEntry;
use objecthash::{self, ObjectHash, ObjectHasher};
use op::{self, Op};
use path::PathBuf;
use proto::ToProto;
use signature::KeyPair;
use timestamp::Timestamp;

const DIGEST_SIZE: usize = 32;
const SIGNATURE_SIZE: usize = 64;
const ADMIN_KEYPAIR_LIFETIME: u64 = 315_532_800; // 10 years

// Block IDs are presently SHA-256 only
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct Id([u8; DIGEST_SIZE]);

impl Id {
    // ID of the genesis block (256-bits of zero)
    pub fn root() -> Id {
        Id([0u8; 32])
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Id> {
        if bytes.len() != DIGEST_SIZE {
            return Err(Error::Parse);
        }

        let mut id = [0u8; DIGEST_SIZE];
        id.copy_from_slice(&bytes[0..DIGEST_SIZE]);

        Ok(Id(id))
    }
}

impl AsRef<[u8]> for Id {
    #[inline(always)]
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl ObjectHash for Id {
    fn objecthash<H: ObjectHasher>(&self, hasher: &mut H) {
        self.0.objecthash(hasher);
    }
}

pub struct Block {
    pub id: Id,
    pub parent_id: Id,
    pub timestamp: Timestamp,
    pub ops: Vec<Op>,
    pub comment: String,
    pub signed_by: [u8; DIGEST_SIZE],
    pub signature: [u8; SIGNATURE_SIZE],
}

impl Block {
    // Create the "genesis block": the first block in the log
    // This block contains the initial administrative signature key which will
    // be used as the initial root authority for new blocks in the log
    // We also sign the genesis block using this key
    pub fn genesis_block(logid: &log::Id,
                         admin_username: &str,
                         admin_keypair: &KeyPair,
                         admin_keypair_sealed: &[u8],
                         comment: &str,
                         digest_alg: DigestAlgorithm,
                         encryption_alg: EncryptionAlgorithm,
                         signature_alg: SignatureAlgorithm)
                         -> Block {
        // SHA256 is the only algorithm we presently support
        assert!(digest_alg == DigestAlgorithm::Sha256);

        let timestamp = Timestamp::now();
        let mut ops = Vec::new();
        let mut path = PathBuf::new();

        ops.push(Op::new(op::Type::Add,
                         path.clone(),
                         Object::Root(RootEntry::new(*logid))));

        let system_ou = OrgUnitEntry::new(Some(String::from("Core system users")));

        path.push("system");
        ops.push(Op::new(op::Type::Add, path.clone(), Object::OrgUnit(system_ou)));

        let admin_user = SystemEntry::new(String::from(admin_username));

        path.push(&admin_username);
        ops.push(Op::new(op::Type::Add, path.clone(), Object::System(admin_user)));

        let admin_keys_ou = OrgUnitEntry::new(Some(String::from("Admin credentials")));

        path.push("keys");
        ops.push(Op::new(op::Type::Add, path.clone(), Object::OrgUnit(admin_keys_ou)));

        let admin_signing_credential =
            CredentialEntry::from_signature_keypair(encryption_alg,
                                                    signature_alg,
                                                    admin_keypair_sealed,
                                                    admin_keypair.public_key_bytes(),
                                                    timestamp,
                                                    timestamp.extend(ADMIN_KEYPAIR_LIFETIME),
                                                    Some(String::from("Root signing key")));

        path.push("signing");
        ops.push(Op::new(op::Type::Add,
                         path,
                         Object::Credential(admin_signing_credential)));

        Block::new(Id::root(), timestamp, ops, comment, admin_keypair)
    }

    pub fn new(parent: Id,
               timestamp: Timestamp,
               ops: Vec<Op>,
               comment: &str,
               keypair: &KeyPair)
               -> Block {
        let mut signed_by = [0u8; 32];
        signed_by.copy_from_slice(&keypair.public_key_bytes());

        let mut block = Block {
            id: Id::root(),
            parent_id: parent,
            timestamp: timestamp,
            ops: ops,
            comment: String::from(comment),
            signed_by: signed_by,
            signature: [0u8; SIGNATURE_SIZE],
        };

        let id = Id::from_bytes(objecthash::digest(&block).as_ref()).unwrap();

        block.id = id;
        block.signature.copy_from_slice(&keypair.sign(id.as_ref()).as_slice());

        block
    }

    pub fn build_json(&self, builder: ObjectBuilder) -> ObjectBuilder {
        builder.insert("id", self.id.as_ref().to_base64(base64::URL_SAFE))
            .insert("parent",
                    self.parent_id.as_ref().to_base64(base64::URL_SAFE))
            .insert("timestamp", self.timestamp)
            .insert_array("ops", |builder| {
                self.ops.iter().fold(builder, |b, op| b.push_object(|b| op.build_json(b)))
            })
            .insert("comment", self.comment.clone())
            .insert("signed_by", self.signed_by.to_base64(base64::URL_SAFE))
            .insert("signature", self.signature.to_base64(base64::URL_SAFE))
    }

    pub fn to_json(&self) -> String {
        let value = self.build_json(ObjectBuilder::new()).build();

        serde_json::to_string(&value).unwrap()
    }
}

impl ToProto for Block {}

impl Serialize for Block {
    fn serialize<O: OutputStream>(&self, out: &mut O) -> io::Result<()> {
        try!(out.write(1, self.id.as_ref()));
        try!(out.write(2, self.parent_id.as_ref()));
        try!(out.write(3, &self.timestamp));
        try!(out.write_repeated(4, &self.ops));
        try!(out.write(5, &self.comment));
        try!(out.write(6, &self.signed_by[..]));
        try!(out.write(7, &self.signature[..]));
        Ok(())
    }
}

impl ObjectHash for Block {
    #[inline]
    fn objecthash<H: ObjectHasher>(&self, hasher: &mut H) {
        objecthash_struct!(
            hasher,
            "parent" => self.parent_id,
            "timestamp" => self.timestamp,
            "ops" => self.ops,
            "comment" => self.comment
        )
    }
}

#[cfg(test)]
pub mod tests {
    use buffoon;
    use ring::rand;

    use algorithm::{DigestAlgorithm, EncryptionAlgorithm, SignatureAlgorithm};
    use block::Block;
    use log;
    use signature::KeyPair;

    const ADMIN_USERNAME: &'static str = "manager";
    const ADMIN_KEYPAIR_SEALED: &'static [u8; 11] = b"placeholder";

    pub fn example_log_id() -> log::Id {
        log::Id::from_bytes(&[0u8; 16]).unwrap()
    }

    pub fn example_block() -> Block {
        let rng = rand::SystemRandom::new();
        let admin_keypair = KeyPair::generate(&rng);

        Block::genesis_block(&example_log_id(),
                             ADMIN_USERNAME,
                             &admin_keypair,
                             ADMIN_KEYPAIR_SEALED,
                             "Initial block",
                             DigestAlgorithm::Sha256,
                             EncryptionAlgorithm::Aes256Gcm,
                             SignatureAlgorithm::Ed25519)
    }

    #[test]
    fn test_proto_serialization() {
        let block = example_block();
        // TODO: better test of the serialized proto
        buffoon::serialize(&block).unwrap();
    }

    #[test]
    fn test_json_serialization() {
        // TODO: better test of the serialized JSON
        example_block().to_json();
    }
}
