use std::io;
use std::string::ToString;

use buffoon::{OutputStream, Serialize};
use rustc_serialize::base64::{self, ToBase64};
use serde_json;
use serde_json::builder::ObjectBuilder;
use time;

use algorithm::DigestAlgorithm;
use error::{Error, Result};
use log;
use objectclass::ObjectClass;
use objectclass::root::RootObject;
use objecthash::{self, ObjectHash, ObjectHasher};
use op::{Op, OpType};
use path::Path;
use proto::ToProto;
use signature::KeyPair;

const DIGEST_SIZE: usize = 32;
const SIGNATURE_SIZE: usize = 64;

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
    pub parent: Id,
    pub timestamp: u64,
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
                         digest_alg: DigestAlgorithm)
                         -> Block {
        // SHA256 is the only algorithm we presently support
        assert!(digest_alg == DigestAlgorithm::SHA256);

        let mut ops = Vec::new();

        ops.push(Op::new(OpType::Add,
                         Path::new("/").unwrap(),
                         ObjectClass::Root(RootObject::new(*logid))));

        ops.push(Op::new(OpType::Add, Path::new("/system").unwrap(), ObjectClass::Ou));

        let public_key_bytes = admin_keypair.public_key_bytes();

        // TODO: replace with e.g. protos
        // let mut admin_user = Vec::with_capacity(public_key_bytes.len() +
        //                                        admin_username.as_bytes().len());
        // admin_user.extend(public_key_bytes);
        // admin_user.extend(admin_username.as_bytes());

        // TODO: add features for path concatenation to the Path type!
        let admin_path = format!("/system/{username}", username = admin_username);

        ops.push(Op::new(OpType::Add,
                         Path::new(&admin_path).unwrap(),
                         ObjectClass::System));

        let admin_keypair_path = format!("{base}/keypair", base = admin_path);

        ops.push(Op::new(OpType::Add,
                         Path::new(&admin_keypair_path).unwrap(),
                         ObjectClass::Credential));
        // &admin_keypair_sealed));

        Block::new(Id::root(),
                   time::now_utc().to_timespec().sec as u64,
                   ops,
                   comment,
                   admin_keypair)
    }

    pub fn new(parent: Id,
               timestamp: u64,
               ops: Vec<Op>,
               comment: &str,
               keypair: &KeyPair)
               -> Block {
        let mut signed_by = [0u8; 32];
        signed_by.copy_from_slice(&keypair.public_key_bytes());

        let mut block = Block {
            id: Id::root(),
            parent: parent,
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

    pub fn to_json(&self) -> String {
        let value = ObjectBuilder::new()
            .insert("id", self.id.as_ref().to_base64(base64::URL_SAFE))
            .insert("parent", self.parent.as_ref().to_base64(base64::URL_SAFE))
            .insert("timestamp", self.timestamp)
            .insert_array("ops", |builder| {
                self.ops.iter().fold(builder, |b, op| {
                    b.push_object(|b| {
                        b.insert("optype", op.optype.to_string())
                            .insert("path", op.path.to_string())
                            .insert("objectclass", op.objectclass.to_string())
                        // TODO: JSON serialization support
                        // .insert("data", op.data.to_base64(base64::URL_SAFE))
                    })
                })

            })
            .insert("comment", self.comment.clone())
            .insert("signed_by", self.signed_by.to_base64(base64::URL_SAFE))
            .insert("signature", self.signature.to_base64(base64::URL_SAFE))
            .build();

        serde_json::to_string(&value).unwrap()
    }
}

impl ToProto for Block {}

impl Serialize for Block {
    fn serialize<O: OutputStream>(&self, out: &mut O) -> io::Result<()> {
        try!(out.write(1, self.id.as_ref()));
        try!(out.write(2, self.parent.as_ref()));
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
            "parent" => self.parent,
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

    use algorithm::DigestAlgorithm;
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
                             DigestAlgorithm::SHA256)
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
