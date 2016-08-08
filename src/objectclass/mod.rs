pub mod domain;
pub mod root;

use std::io;
use std::string::ToString;

use buffoon::{Serialize, OutputStream};
use ring::digest::Digest;

use objecthash::ObjectHash;
use proto::ToProto;

use self::root::RootObject;
use self::domain::DomainObject;

#[derive(Debug, Eq, PartialEq)]
pub enum ObjectClass {
    Root(RootObject), // Root DSE
    Domain(DomainObject), // ala DNS domain or Kerberos realm
    Ou, // Organizational unit
    Credential, // Encrypted access credential
    System, // System User (i.e. non-human account)
    Host, // an individual server
}

impl ObjectClass {
    pub fn protobuf_id(&self) -> u32 {
        match *self {
            ObjectClass::Root(_) => 1,
            ObjectClass::Domain(_) => 2,
            ObjectClass::Ou => 3,
            ObjectClass::Credential => 4,
            ObjectClass::System => 5,
            ObjectClass::Host => 6,
        }
    }
}

impl ToProto for ObjectClass {}

impl ToString for ObjectClass {
    fn to_string(&self) -> String {
        match *self {
            ObjectClass::Root(_) => "ROOT".to_string(),
            ObjectClass::Domain(_) => "DOMAIN".to_string(),
            ObjectClass::Ou => "OU".to_string(),
            ObjectClass::Credential => "CREDENTIAL".to_string(),
            ObjectClass::System => "SYSTEM".to_string(),
            ObjectClass::Host => "HOST".to_string(),
        }
    }
}

impl Serialize for ObjectClass {
    fn serialize<O: OutputStream>(&self, out: &mut O) -> io::Result<()> {
        try!(out.write(1, &self.protobuf_id()));

        let object_proto = match self {
            &ObjectClass::Root(ref root) => root.to_proto(),
            &ObjectClass::Domain(ref domain) => domain.to_proto(),
            _ => Ok(Vec::new()), // TODO
        };

        if !object_proto.is_ok() {
            return Err(io::Error::new(io::ErrorKind::InvalidData,
                                      format!("couldn't serialize {type}", type=self.to_string())));
        }

        try!(out.write(2, &object_proto.unwrap()));

        Ok(())
    }
}

impl ObjectHash for ObjectClass {
    fn objecthash(&self) -> Digest {
        match self {
            &ObjectClass::Root(ref root) => root.objecthash(),
            &ObjectClass::Domain(ref domain) => domain.objecthash(),
            _ => "".objecthash(), // TODO
        }
    }
}
