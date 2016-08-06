use std::io;
use std::string::ToString;

use buffoon::{self, OutputStream, Serialize};
use ring::digest;
use rustc_serialize::base64::{self, ToBase64};
use serde_json;
use serde_json::builder::ObjectBuilder;
use time;

use error::{Error, Result};
use objectclass::ObjectClass;
use objecthash::{ObjectHash, DIGEST_ALG};
use op::{Op, OpType};
use path::Path;
use signature::KeyPair;

const DIGEST_SIZE: usize = 32;
const SIGNATURE_SIZE: usize = 64;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum DigestAlgorithm {
    SHA256,
}

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
    #[inline]
    fn objecthash(&self) -> digest::Digest {
        self.0.objecthash()
    }
}

pub struct Block {
    pub id: Option<Id>,
    pub parent: Id,
    pub timestamp: u64,
    pub ops: Vec<Op>,
    pub comment: String,
    pub signed_by: Option<[u8; DIGEST_SIZE]>,
    pub signature: Option<[u8; SIGNATURE_SIZE]>,
}

impl Block {
    // Create the "genesis block": the first block in the log
    // This block contains the initial administrative signature key which will
    // be used as the initial root authority for new blocks in the log
    // We also sign the genesis block using this key
    pub fn genesis_block(logid: &[u8; 16],
                         admin_username: &str,
                         admin_keypair: &KeyPair,
                         admin_keypair_sealed: &[u8],
                         comment: &str,
                         digest_alg: DigestAlgorithm)
                         -> Block {
        let mut block = Block::new(Id::root());

        // TODO: use a real type for the root entry
        block.op(OpType::Add,
                 Path::new("/").unwrap(),
                 ObjectClass::Root,
                 logid);

        block.op(OpType::Add,
                 Path::new("/system").unwrap(),
                 ObjectClass::Ou,
                 b"");

        let public_key_bytes = admin_keypair.public_key_bytes();

        // TODO: replace with e.g. protos
        let mut admin_user = Vec::with_capacity(public_key_bytes.len() +
                                                admin_username.as_bytes().len());
        admin_user.extend(public_key_bytes);
        admin_user.extend(admin_username.as_bytes());

        // TODO: add features for path concatenation to the Path type!
        let admin_path = format!("/system/{username}", username = admin_username);

        block.op(OpType::Add,
                 Path::new(&admin_path).unwrap(),
                 ObjectClass::System,
                 &admin_user);

        let admin_keypair_path = format!("{base}/keypair", base = admin_path);

        block.op(OpType::Add,
                 Path::new(&admin_keypair_path).unwrap(),
                 ObjectClass::Credential,
                 &admin_keypair_sealed);

        block.comment.push_str(comment);

        block.sign(admin_keypair, digest_alg);

        block
    }

    pub fn new(parent: Id) -> Block {
        Block {
            id: None,
            parent: parent,
            timestamp: time::now_utc().to_timespec().sec as u64,
            ops: Vec::new(),
            comment: String::new(),
            signed_by: None,
            signature: None,
        }
    }

    pub fn op(&mut self, optype: OpType, path: Path, objectclass: ObjectClass, data: &[u8]) {
        self.ops.push(Op::new(optype, path, objectclass, data));
    }

    pub fn sign(&mut self, keypair: &KeyPair, digest_alg: DigestAlgorithm) {
        // SHA-256 is the only digest algorithm we support for now
        assert!(digest_alg == DigestAlgorithm::SHA256);

        let mut signed_by = [0u8; 32];
        signed_by.copy_from_slice(&keypair.public_key_bytes());
        self.signed_by = Some(signed_by);

        let id = Id::from_bytes(self.objecthash().as_ref()).unwrap();
        self.id = Some(id);

        let mut signature = [0u8; 64];
        signature.copy_from_slice(&keypair.sign(id.as_ref()).as_slice());
        self.signature = Some(signature);
    }

    pub fn to_proto(&self) -> Result<Vec<u8>> {
        buffoon::serialize(&self).map_err(|_| Error::Serialize)
    }

    pub fn to_json(&self) -> String {
        let value = ObjectBuilder::new()
            .insert("id",
                    self.id
                        .expect("id missing")
                        .as_ref()
                        .to_base64(base64::URL_SAFE))
            .insert("parent", self.parent.as_ref().to_base64(base64::URL_SAFE))
            .insert("timestamp", self.timestamp)
            .insert_array("ops", |builder| {
                self.ops.iter().fold(builder, |b, op| {
                    b.push_object(|b| {
                        b.insert("optype", op.optype.to_string())
                            .insert("path", op.path.to_string())
                            .insert("objectclass", op.objectclass.to_string())
                            .insert("data", op.data.to_base64(base64::URL_SAFE))
                    })
                })

            })
            .insert("comment", self.comment.clone())
            .insert("signed_by",
                    self.signed_by.expect("signed_by missing").to_base64(base64::URL_SAFE))
            .insert("signature",
                    self.signature.expect("signature missing").to_base64(base64::URL_SAFE))
            .build();

        serde_json::to_string(&value).unwrap()
    }
}

impl Serialize for Block {
    fn serialize<O: OutputStream>(&self, out: &mut O) -> io::Result<()> {
        try!(out.write(1, self.id.expect("id missing").as_ref()));
        try!(out.write(2, self.parent.as_ref()));
        try!(out.write(3, &self.timestamp));
        try!(out.write_repeated(4, &self.ops));
        try!(out.write(5, &self.comment));
        try!(out.write(6, &self.signed_by.expect("signed_by missing")[..]));
        try!(out.write(7, &self.signature.expect("signature missing")[..]));
        Ok(())
    }
}

impl ObjectHash for Block {
    #[inline]
    fn objecthash(&self) -> digest::Digest {
        let mut block_ctx = digest::Context::new(&DIGEST_ALG);

        // objecthash qualifier for dictionaries
        block_ctx.update(b"d");

        block_ctx.update("parent".objecthash().as_ref());
        block_ctx.update(self.parent.objecthash().as_ref());

        block_ctx.update("timestamp".objecthash().as_ref());
        block_ctx.update(self.timestamp.objecthash().as_ref());

        block_ctx.update("ops".objecthash().as_ref());
        block_ctx.update(self.ops.objecthash().as_ref());

        block_ctx.update("comment".objecthash().as_ref());
        block_ctx.update(self.comment.objecthash().as_ref());

        block_ctx.update("signed_by".objecthash().as_ref());
        block_ctx.update(self.signed_by.expect("signed_by missing").objecthash().as_ref());

        block_ctx.finish()
    }
}

#[cfg(test)]
pub mod tests {
    use buffoon;
    use ring::rand;

    use block::{Block, DigestAlgorithm};
    use signature::KeyPair;

    const LOGID: &'static [u8; 16] = &[0u8; 16];
    const ADMIN_USERNAME: &'static str = "manager";
    const ADMIN_KEYPAIR_SEALED: &'static [u8; 11] = b"placeholder";

    pub fn example_block() -> Block {
        let rng = rand::SystemRandom::new();
        let admin_keypair = KeyPair::generate(&rng);

        Block::genesis_block(LOGID,
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
