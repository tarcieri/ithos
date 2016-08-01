use std::{self, mem, str};
use std::hash::{Hash, Hasher};
use std::collections::HashMap;

use ring::rand;

use adapter::{Adapter, Transaction};
use error::{Error, Result};
use lmdb_adapter::LmdbAdapter;
use log::{OpType, Block, DigestAlgorithm};
use password::{self, PasswordAlgorithm};
use objectclass::ObjectClass;
use signature::{SignatureAlgorithm, KeyPair};

#[cfg(test)]
extern crate tempdir;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct Id(u64);

#[derive(Debug, Eq, PartialEq)]
pub struct Node<'a> {
    pub id: Id,
    pub parent_id: Id,
    pub name: &'a str,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Entry<'a> {
    pub node: Node<'a>,
    pub objectclass: ObjectClass,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Path {
    pub components: Vec<String>,
}

pub struct Server {
    adapter: LmdbAdapter,
}

// Ids are 64-bit integers in host-native byte order
// LMDB has special optimizations for host-native integers as keys
impl Id {
    pub fn root() -> Id {
        Id(0)
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Id> {
        if bytes.len() != 8 {
            return Err(Error::Parse);
        }

        let mut id = [0u8; 8];
        id.copy_from_slice(&bytes[0..8]);

        Ok(Id(unsafe { mem::transmute(id) }))
    }

    pub fn as_bytes(self) -> [u8; 8] {
        let Id(id) = self;
        unsafe { mem::transmute(id) }
    }

    pub fn next(self) -> Id {
        let Id(id) = self;
        Id(id + 1)
    }
}

impl<'a> Node<'a> {
    pub fn root() -> Node<'a> {
        Node {
            id: Id::root(),
            parent_id: Id::root(),
            name: "/",
        }
    }

    pub fn from_parent_id_and_bytes(parent_id: Id, bytes: &[u8]) -> Result<Node> {
        if bytes.len() < 8 {
            return Err(Error::DbCorrupt);
        }

        let id = try!(Id::from_bytes(&bytes[0..8]));
        let name = try!(str::from_utf8(&bytes[8..]).map_err(|_| Error::DbCorrupt));

        Ok(Node {
            id: id,
            parent_id: parent_id,
            name: name,
        })
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(8 + self.name.len());
        bytes.extend_from_slice(&self.id.as_bytes());
        bytes.extend_from_slice(self.name.as_bytes());
        bytes
    }
}

impl Path {
    pub fn new(string: &str) -> Result<Path> {
        let mut components: Vec<String> =
            string.split("/").map(|component| String::from(component)).collect();

        if components.is_empty() {
            return Err(Error::PathInvalid);
        }

        let prefix = components.remove(0);

        // Does the path start with something other than "/"?
        if !prefix.is_empty() {
            return Err(Error::PathInvalid);
        }

        Ok(Path { components: components })
    }

    pub fn to_string(&self) -> String {
        let mut result = String::new();

        for component in self.components.clone() {
            result.push_str("/");
            result.push_str(&component);
        }

        result
    }

    pub fn parent(&self) -> Path {
        if self.is_root() {
            return Path { components: vec![String::from("")] };
        }

        let mut parent_components = self.components.clone();
        parent_components.pop();

        Path { components: parent_components }
    }

    pub fn name(&self) -> String {
        self.components.last().unwrap().clone()
    }

    pub fn is_root(&self) -> bool {
        self.components.len() == 1 && self.components[0] == ""
    }
}

impl Hash for Path {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for component in &self.components {
            component.hash(state);
        }
    }
}

impl Server {
    pub fn create_database(path: &std::path::Path,
                           admin_username: &str,
                           admin_password: &str)
                           -> Result<Server> {
        let rng = rand::SystemRandom::new();
        let mut logid = [0u8; 16];
        try!(rng.fill(&mut logid).map_err(|_| Error::Rng));

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
                                                 DigestAlgorithm::SHA256);

        let adapter = LmdbAdapter::create_database(path).unwrap();
        let server = Server { adapter: adapter };

        try!(server.commit_unverified_block(&genesis_block));
        Ok(server)
    }

    // Commit a block without first checking its signature
    fn commit_unverified_block(&self, block: &Block) -> Result<()> {
        let mut txn = try!(self.adapter.rw_transaction());
        let mut id = try!(self.adapter.next_available_id(&txn));
        let mut new_entries = HashMap::new();

        // Process the operations in the block and apply them to the database
        for op in &block.ops {
            match op.optype {
                OpType::Add => {
                    let parent_id = if op.path.is_root() {
                        Id::root()
                    } else {
                        match new_entries.get(&op.path.parent()) {
                            Some(&id) => id,
                            _ => try!(self.adapter.find_node(&txn, &op.path.parent())).id,
                        }
                    };

                    let name = op.path.name();

                    // NOTE: The underlying adapter must handle Error::EntryAlreadyExists
                    try!(self.adapter.add_entry(&mut txn, id, parent_id, &name, op.objectclass));
                    new_entries.insert(&op.path, id);
                    id = id.next()
                }
            }
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
