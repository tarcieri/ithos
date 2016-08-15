pub mod domain;
pub mod ou;
pub mod root;

use std::io;
use std::string::ToString;

use buffoon::{Serialize, OutputStream};

use objecthash::{ObjectHash, ObjectHasher};
use proto::ToProto;

use self::domain::DomainObject;
use self::ou::OrganizationalUnitObject;
use self::root::RootObject;

#[derive(Debug, Eq, PartialEq)]
pub enum ObjectClass {
    Root(RootObject), // Root entry in the tree (ala LDAP root DSE)
    Domain(DomainObject), // Administrative Domain (ala DNS domain or Kerberos realm)
    OrganizationalUnit(OrganizationalUnitObject), // Unit/department within an organization
    Credential, // Encrypted access credential
    System, // System User (i.e. non-human account)
    Host, // Individual server within a domain
}

impl ObjectClass {
    pub fn protobuf_id(&self) -> u32 {
        match *self {
            ObjectClass::Root(_) => 1,
            ObjectClass::Domain(_) => 2,
            ObjectClass::OrganizationalUnit(_) => 3,
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
            ObjectClass::OrganizationalUnit(_) => "ORGANIZATIONAL_UNIT".to_string(),
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
            &ObjectClass::OrganizationalUnit(ref ou) => ou.to_proto(),
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
    fn objecthash<H: ObjectHasher>(&self, hasher: &mut H) {
        match self {
            &ObjectClass::Root(ref root) => root.objecthash(hasher),
            &ObjectClass::Domain(ref domain) => domain.objecthash(hasher),
            _ => (), // TODO
        }
    }
}
