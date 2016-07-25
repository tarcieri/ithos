use std::string::ToString;
use ring::digest::Digest;

use objecthash::ObjectHash;
use server::{Result, Error};

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum ObjectClass {
    ROOT, // Root DSE
    DOMAIN, // ala DNS domain or Kerberos realm
    OU, // Organizational unit
    SYSTEM, // System User (i.e. non-human account)
    HOST, // an individual server
}

impl ObjectClass {
    pub fn from_bytes(bytes: &[u8]) -> Result<ObjectClass> {
        match bytes {
            b"root" => Ok(ObjectClass::ROOT),
            b"domain" => Ok(ObjectClass::DOMAIN),
            b"ou" => Ok(ObjectClass::OU),
            b"system" => Ok(ObjectClass::SYSTEM),
            b"host" => Ok(ObjectClass::HOST),
            _ => Err(Error::NotFoundError),
        }
    }
}

impl ToString for ObjectClass {
    fn to_string(&self) -> String {
        match *self {
            ObjectClass::ROOT => "root".to_string(),
            ObjectClass::DOMAIN => "domain".to_string(),
            ObjectClass::OU => "ou".to_string(),
            ObjectClass::HOST => "host".to_string(),
            ObjectClass::SYSTEM => "system".to_string(),
        }
    }
}

impl ObjectHash for ObjectClass {
    fn objecthash(&self) -> Digest {
        self.to_string().objecthash()
    }
}
