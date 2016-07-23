// "Merkelized" append-only replication log

extern crate time;

use ring::digest;
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
    objectclass: String,
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

pub struct Log {
    id: [u8; 16],
    signature_alg: SignatureAlgorithm,
    digest_alg: DigestAlgorithm,
    head: [u8; 32],
    blocks: Vec<Block>,
}

impl Op {
    pub fn new(optype: OpType, path: &str, objectclass: &str, data: &[u8]) -> Op {
        Op {
            optype: optype,
            path: String::from(path),
            objectclass: String::from(objectclass),
            data: Vec::from(data),
        }
    }

    // TODO: abstraction!
    pub fn objecthash(&self, algorithm: DigestAlgorithm) -> digest::Digest {
        // SHA-256 is the only digest algorithm we support for now
        assert!(algorithm == DigestAlgorithm::SHA256);

        let mut op_ctx = digest::Context::new(&digest::SHA256);
        op_ctx.update(b"d"); // objecthash qualifier for dictionaries

        // OpType::Add is the only op we support right now
        assert!(self.optype == OpType::Add);
        let mut optype_ctx = digest::Context::new(&digest::SHA256);

        {
            let optype_str = "ADD";

            optype_ctx.update(b"u"); // objecthash qualifier for Unicode strings
            optype_ctx.update(digest::digest(&digest::SHA256, &optype_str.as_bytes()).as_ref());
        }

        op_ctx.update(digest::digest(&digest::SHA256, b"optype").as_ref());
        op_ctx.update(optype_ctx.finish().as_ref());

        let mut path_ctx = digest::Context::new(&digest::SHA256);

        {
            path_ctx.update(b"u"); // objecthash qualifier for Unicode strings
            path_ctx.update(digest::digest(&digest::SHA256, &self.path.as_bytes()).as_ref());
        }

        op_ctx.update(digest::digest(&digest::SHA256, b"path").as_ref());
        op_ctx.update(path_ctx.finish().as_ref());

        let mut objectclass_ctx = digest::Context::new(&digest::SHA256);

        {
            objectclass_ctx.update(b"u"); // objecthash qualifier for Unicode strings
            objectclass_ctx.update(digest::digest(
                &digest::SHA256,
                &self.objectclass.as_bytes()
            ).as_ref());
        }

        op_ctx.update(digest::digest(&digest::SHA256, b"objectclass").as_ref());
        op_ctx.update(objectclass_ctx.finish().as_ref());

        // TODO: we should serialize the underlying structure of the data
        // That way we can transcode it between e.g. protos and JSON
        let mut data_ctx = digest::Context::new(&digest::SHA256);

        {
            data_ctx.update(b"u"); // objecthash qualifier for Unicode strings
            data_ctx.update(digest::digest(&digest::SHA256, &self.data).as_ref());
        }

        op_ctx.update(digest::digest(&digest::SHA256, b"data").as_ref());
        op_ctx.update(data_ctx.finish().as_ref());

        op_ctx.finish()
    }
}

impl OobData {
    pub fn new(label: &str, data: &[u8]) -> OobData {
        OobData {
            label: String::from(label),
            data: Vec::from(data),
        }
    }

    pub fn objecthash(&self, algorithm: DigestAlgorithm) -> digest::Digest {
        // SHA-256 is the only digest algorithm we support for now
        assert!(algorithm == DigestAlgorithm::SHA256);

        let mut oob_data_ctx = digest::Context::new(&digest::SHA256);
        oob_data_ctx.update(b"d"); // objecthash qualifier for dictionaries

        let mut label_ctx = digest::Context::new(&digest::SHA256);

        {
            label_ctx.update(b"u"); // objecthash qualifier for Unicode strings
            label_ctx.update(digest::digest(&digest::SHA256, &self.label.as_bytes()).as_ref());
        }

        oob_data_ctx.update(digest::digest(&digest::SHA256, b"label").as_ref());
        oob_data_ctx.update(label_ctx.finish().as_ref());

        let mut data_ctx = digest::Context::new(&digest::SHA256);

        {
            data_ctx.update(b"u"); // objecthash qualifier for Unicode strings
            data_ctx.update(digest::digest(&digest::SHA256, &self.data).as_ref());
        }

        oob_data_ctx.update(digest::digest(&digest::SHA256, b"data").as_ref());
        oob_data_ctx.update(data_ctx.finish().as_ref());

        oob_data_ctx.finish()
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
                         admin_keypair_sealed: &[u8])
                         -> Block {
        let mut block = Block::new(GENESIS_BLOCK_ID);

        block.op(OpType::Add, "/id", "logid", logid);

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

        block.op(OpType::Add, &admin_path, "system", &admin_user);

        let mut keypair_label = String::new();
        keypair_label.push_str(&admin_username);
        keypair_label.push_str(".keypair");

        block.oob_data(&keypair_label, &admin_keypair_sealed);

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

    pub fn op(&mut self, optype: OpType, path: &str, objectclass: &str, data: &[u8]) {
        self.ops.push(Op::new(optype, path, objectclass, data));
    }

    pub fn oob_data(&mut self, label: &str, data: &[u8]) {
        self.oob_data.push(OobData::new(&label, &data));
    }

    // TODO: abstraction!
    pub fn objecthash(&self, algorithm: DigestAlgorithm) -> digest::Digest {
        // SHA-256 is the only digest algorithm we support for now
        assert!(algorithm == DigestAlgorithm::SHA256);

        let mut block_ctx = digest::Context::new(&digest::SHA256);
        block_ctx.update(b"d"); // objecthash qualifier for dictionaries

        let mut timestamp_ctx = digest::Context::new(&digest::SHA256);

        {
            timestamp_ctx.update(b"i"); // objecthash qualifier for integers
            timestamp_ctx.update(self.timestamp.to_string().as_bytes());
        }

        block_ctx.update(digest::digest(&digest::SHA256, b"timestamp").as_ref());
        block_ctx.update(timestamp_ctx.finish().as_ref());

        let mut op_list_ctx = digest::Context::new(&digest::SHA256);

        {
            op_list_ctx.update(b"l"); // objecthash qualifier for lists

            let ref ops = self.ops;
            for op in ops {
                op_list_ctx.update(op.objecthash(algorithm).as_ref());
            }
        }

        block_ctx.update(digest::digest(&digest::SHA256, b"ops").as_ref());
        block_ctx.update(op_list_ctx.finish().as_ref());

        let mut oob_data_ctx = digest::Context::new(&digest::SHA256);

        {
            oob_data_ctx.update(b"d"); // objecthash qualifier for lists

            let ref oob_data = self.oob_data;
            for o in oob_data {
                oob_data_ctx.update(o.objecthash(algorithm).as_ref());
            }
        }

        block_ctx.update(digest::digest(&digest::SHA256, b"oob_data").as_ref());
        block_ctx.update(oob_data_ctx.finish().as_ref());

        let mut comment_ctx = digest::Context::new(&digest::SHA256);

        {
            comment_ctx.update(b"u"); // objecthash qualifier for Unicode strings
            comment_ctx.update(digest::digest(&digest::SHA256, &self.comment.as_bytes()).as_ref());
        }

        block_ctx.update(digest::digest(&digest::SHA256, b"comment").as_ref());
        block_ctx.update(comment_ctx.finish().as_ref());

        let mut signed_by_ctx = digest::Context::new(&digest::SHA256);

        {
            signed_by_ctx.update(b"u"); // objecthash qualifier for Unicode strings

            // TODO: Don't Panic
            let signed_by = self.signed_by.expect("signed_by missing");
            signed_by_ctx.update(digest::digest(&digest::SHA256, &signed_by).as_ref());
        }

        block_ctx.update(digest::digest(&digest::SHA256, b"signed_by").as_ref());
        block_ctx.update(signed_by_ctx.finish().as_ref());

        block_ctx.finish()
    }

    pub fn sign(&mut self, keypair: &KeyPair, digest_algorithm: DigestAlgorithm) {
        let mut signed_by = [0u8; 32];
        signed_by.copy_from_slice(&keypair.public_key_bytes());
        self.signed_by = Some(signed_by);

        let mut id = [0u8; 32];
        id.copy_from_slice(self.objecthash(digest_algorithm).as_ref());
        self.id = Some(id);

        let mut signature = [0u8; 64];
        signature.copy_from_slice(keypair.sign(&id).as_slice());
        self.signature = Some(signature);
    }
}

impl Log {
    pub fn generate(logid: &[u8; 16],
                    admin_username: &str,
                    admin_keypair: &KeyPair,
                    admin_keypair_sealed: &[u8],
                    digest_algorithm: DigestAlgorithm)
                    -> Log {
        let mut genesis_block =
            Block::genesis_block(logid, admin_username, admin_keypair, admin_keypair_sealed);

        genesis_block.sign(admin_keypair, digest_algorithm);

        Log {
            id: *logid,
            signature_alg: admin_keypair.algorithm(),
            digest_alg: digest_algorithm,
            head: *GENESIS_BLOCK_ID,
            blocks: vec![genesis_block],
        }
    }
}

#[cfg(test)]
mod tests {
    use ring::rand;

    use log::{Log, DigestAlgorithm};
    use signature::{SignatureAlgorithm, KeyPair};

    const LOGID: &'static [u8; 16] = &[0u8; 16];
    const ADMIN_USERNAME: &'static str = "manager";
    const ADMIN_KEYPAIR_SEALED: &'static [u8; 11] = b"placeholder";

    #[test]
    fn test_generate_log() {
        let rng = rand::SystemRandom::new();
        let admin_keypair = KeyPair::generate(&rng);

        let log = Log::generate(LOGID,
                                ADMIN_USERNAME,
                                &admin_keypair,
                                ADMIN_KEYPAIR_SEALED,
                                DigestAlgorithm::SHA256);
    }
}
