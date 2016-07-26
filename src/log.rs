// "Merkelized" append-only replication log

use time;
use ring::digest;

use objectclass::ObjectClass;
use objecthash::{ObjectHash, DIGEST_ALG};
use signature::{SignatureAlgorithm, KeyPair};

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum DigestAlgorithm {
    SHA256,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum OpType {
    Add,
}

pub struct Op {
    optype: OpType,
    path: String,
    objectclass: ObjectClass,
    data: Vec<u8>,
}

pub struct OobData {
    pub label: String,
    pub data: Vec<u8>,
}

pub struct Block {
    id: Option<[u8; 32]>,
    parent: [u8; 32],
    timestamp: u64,
    ops: Vec<Op>,
    oob_data: Vec<OobData>,
    comment: String,
    signed_by: Option<[u8; 32]>,
    signature: Option<[u8; 64]>,
}

impl Op {
    pub fn new(optype: OpType, path: &str, objectclass: ObjectClass, data: &[u8]) -> Op {
        Op {
            optype: optype,
            path: String::from(path),
            objectclass: objectclass,
            data: Vec::from(data),
        }
    }
}

impl ObjectHash for Op {
    fn objecthash(&self) -> digest::Digest {
        let mut ctx = digest::Context::new(&DIGEST_ALG);

        // objecthash qualifier for dictionaries
        ctx.update(b"d");

        // OpType::Add is the only op we support right now
        assert!(self.optype == OpType::Add);
        let optype_str = "ADD";

        ctx.update(digest::digest(&DIGEST_ALG, b"optype").as_ref());
        ctx.update(optype_str.objecthash().as_ref());

        ctx.update(digest::digest(&DIGEST_ALG, b"path").as_ref());
        ctx.update(self.path.objecthash().as_ref());

        ctx.update(digest::digest(&DIGEST_ALG, b"objectclass").as_ref());
        ctx.update(self.objectclass.objecthash().as_ref());

        ctx.update(digest::digest(&DIGEST_ALG, b"data").as_ref());
        ctx.update(self.data.objecthash().as_ref());

        ctx.finish()
    }
}

impl OobData {
    pub fn new(label: &str, data: &[u8]) -> OobData {
        OobData {
            label: String::from(label),
            data: Vec::from(data),
        }
    }
}

impl ObjectHash for OobData {
    fn objecthash(&self) -> digest::Digest {
        let mut ctx = digest::Context::new(&DIGEST_ALG);

        // objecthash qualifier for dictionaries
        ctx.update(b"d");

        ctx.update(digest::digest(&DIGEST_ALG, b"label").as_ref());
        ctx.update(self.label.objecthash().as_ref());

        ctx.update(digest::digest(&DIGEST_ALG, b"data").as_ref());
        ctx.update(self.data.objecthash().as_ref());

        ctx.finish()
    }
}

// ID of the genesis block (256-bits of zero)
pub const GENESIS_BLOCK_ID: &'static [u8; 32] = &[0u8; 32];

impl Block {
    // Create the "genesis block": the first block in the log
    // This block contains the initial administrative signature key which will
    // be used as the initial root authority for new blocks in the log
    // We also sign the genesis block using this key
    pub fn genesis_block(logid: &[u8; 16],
                         admin_username: &str,
                         admin_keypair: &KeyPair,
                         admin_keypair_sealed: &[u8],
                         digest_alg: DigestAlgorithm)
                         -> Block {
        let mut block = Block::new(GENESIS_BLOCK_ID);

        block.op(OpType::Add, "", ObjectClass::Root, logid);

        let public_key_bytes = admin_keypair.public_key_bytes();

        // TODO: replace with e.g. protos
        let mut admin_user = Vec::with_capacity(public_key_bytes.len() +
                                                admin_username.as_bytes().len());
        admin_user.extend(public_key_bytes);
        admin_user.extend(admin_username.as_bytes());

        // TODO: replace with a type for managing paths!
        let mut admin_path = String::new();
        admin_path.push_str("/system/");
        admin_path.push_str(admin_username);

        block.op(OpType::Add, &admin_path, ObjectClass::System, &admin_user);

        let mut keypair_label = String::new();
        keypair_label.push_str(&admin_username);
        keypair_label.push_str(".keypair");

        block.oob_data(&keypair_label, &admin_keypair_sealed);

        // TODO: Customization
        block.comment.push_str("Initial block");

        block.sign(admin_keypair, digest_alg);

        block
    }

    pub fn new(parent: &[u8; 32]) -> Block {
        Block {
            id: None,
            timestamp: time::now_utc().to_timespec().sec as u64,
            parent: *parent,
            ops: Vec::new(),
            oob_data: Vec::new(),
            comment: String::new(),
            signed_by: None,
            signature: None,
        }
    }

    pub fn op(&mut self, optype: OpType, path: &str, objectclass: ObjectClass, data: &[u8]) {
        self.ops.push(Op::new(optype, path, objectclass, data));
    }

    pub fn oob_data(&mut self, label: &str, data: &[u8]) {
        self.oob_data.push(OobData::new(&label, &data));
    }

    pub fn sign(&mut self, keypair: &KeyPair, digest_alg: DigestAlgorithm) {
        // SHA-256 is the only digest algorithm we support for now
        assert!(digest_alg == DigestAlgorithm::SHA256);

        let mut signed_by = [0u8; 32];
        signed_by.copy_from_slice(&keypair.public_key_bytes());
        self.signed_by = Some(signed_by);

        let mut id = [0u8; 32];
        id.copy_from_slice(self.objecthash().as_ref());
        self.id = Some(id);

        let mut signature = [0u8; 64];
        signature.copy_from_slice(keypair.sign(&id).as_slice());
        self.signature = Some(signature);
    }
}

impl ObjectHash for Block {
    fn objecthash(&self) -> digest::Digest {
        let mut block_ctx = digest::Context::new(&DIGEST_ALG);

        // objecthash qualifier for dictionaries
        block_ctx.update(b"d");

        block_ctx.update(digest::digest(&DIGEST_ALG, b"timestamp").as_ref());
        block_ctx.update(self.timestamp.objecthash().as_ref());

        block_ctx.update(digest::digest(&DIGEST_ALG, b"ops").as_ref());
        block_ctx.update(self.ops.objecthash().as_ref());

        block_ctx.update(digest::digest(&DIGEST_ALG, b"oob_data").as_ref());
        block_ctx.update(self.oob_data.objecthash().as_ref());

        block_ctx.update(digest::digest(&DIGEST_ALG, b"comment").as_ref());
        block_ctx.update(self.comment.objecthash().as_ref());

        block_ctx.update(digest::digest(&DIGEST_ALG, b"signed_by").as_ref());
        block_ctx.update(self.signed_by.expect("signed_by missing").objecthash().as_ref());

        block_ctx.finish()
    }
}

#[cfg(test)]
mod tests {
    use ring::rand;

    use log::{Block, DigestAlgorithm};
    use signature::{SignatureAlgorithm, KeyPair};

    const LOGID: &'static [u8; 16] = &[0u8; 16];
    const ADMIN_USERNAME: &'static str = "manager";
    const ADMIN_KEYPAIR_SEALED: &'static [u8; 11] = b"placeholder";

    #[test]
    fn test_generate_log() {
        let rng = rand::SystemRandom::new();
        let admin_keypair = KeyPair::generate(&rng);

        let block = Block::genesis_block(LOGID,
                                         ADMIN_USERNAME,
                                         &admin_keypair,
                                         ADMIN_KEYPAIR_SEALED,
                                         DigestAlgorithm::SHA256);
    }
}
