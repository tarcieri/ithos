use std::string::ToString;
use ring::digest::Digest;

use error::{Error, Result};
use objecthash::ObjectHash;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum ObjectClass {
    Root, // Root DSE
    Domain, // ala DNS domain or Kerberos realm
    Ou, // Organizational unit
    Credential, // Encrypted access credential
    System, // System User (i.e. non-human account)
    Host, // an individual server
}

impl ObjectClass {
    pub fn from_bytes(bytes: &[u8]) -> Result<ObjectClass> {
        match bytes {
            b"root" => Ok(ObjectClass::Root),
            b"domain" => Ok(ObjectClass::Domain),
            b"ou" => Ok(ObjectClass::Ou),
            b"credential" => Ok(ObjectClass::Credential),
            b"system" => Ok(ObjectClass::System),
            b"host" => Ok(ObjectClass::Host),
            _ => Err(Error::NotFound),
        }
    }
}

impl ToString for ObjectClass {
    fn to_string(&self) -> String {
        match *self {
            ObjectClass::Root => "root".to_string(),
            ObjectClass::Domain => "domain".to_string(),
            ObjectClass::Ou => "ou".to_string(),
            ObjectClass::Credential => "credential".to_string(),
            ObjectClass::System => "system".to_string(),
            ObjectClass::Host => "host".to_string(),
        }
    }
}

impl ObjectHash for ObjectClass {
    fn objecthash(&self) -> Digest {
        self.to_string().objecthash()
    }
}
