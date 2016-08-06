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
