pub mod credential;
pub mod domain;
pub mod ou;
pub mod root;
pub mod system;

use std::io;
use std::string::ToString;

use buffoon::{Serialize, OutputStream};
use serde_json::builder::ObjectBuilder;
use objecthash::{ObjectHash, ObjectHasher};

use proto::ToProto;

use self::credential::CredentialObject;
use self::domain::DomainObject;
use self::ou::OrganizationalUnitObject;
use self::root::RootObject;
use self::system::SystemObject;

#[derive(Debug, Eq, PartialEq)]
pub enum ObjectClass {
    Root(RootObject), // Root entry in the tree (ala LDAP root DSE)
    Domain(DomainObject), // Administrative Domain (ala DNS domain or Kerberos realm)
    OrganizationalUnit(OrganizationalUnitObject), // Unit/department within an organization
    System(SystemObject), // System User (i.e. non-human account)
    Credential(CredentialObject), // Encrypted access credential
}

impl ObjectClass {
    pub fn protobuf_id(&self) -> u32 {
        match *self {
            ObjectClass::Root(_) => 1,
            ObjectClass::Domain(_) => 2,
            ObjectClass::OrganizationalUnit(_) => 3,
            ObjectClass::System(_) => 4,
            ObjectClass::Credential(_) => 5,
        }
    }

    pub fn build_json(&self, builder: ObjectBuilder) -> ObjectBuilder {
        builder.insert("type", self.to_string())
            .insert_object("value", |b| match *self {
                ObjectClass::Root(ref root) => root.build_json(b),
                ObjectClass::Domain(ref domain) => domain.build_json(b),
                ObjectClass::OrganizationalUnit(ref ou) => ou.build_json(b),
                ObjectClass::System(ref system) => system.build_json(b),
                ObjectClass::Credential(ref credential) => credential.build_json(b),
            })
    }
}

impl ToProto for ObjectClass {}

impl ToString for ObjectClass {
    fn to_string(&self) -> String {
        match *self {
            ObjectClass::Root(_) => "ROOT".to_string(),
            ObjectClass::Domain(_) => "DOMAIN".to_string(),
            ObjectClass::OrganizationalUnit(_) => "ORGANIZATIONAL_UNIT".to_string(),
            ObjectClass::System(_) => "SYSTEM".to_string(),
            ObjectClass::Credential(_) => "CREDENTIAL".to_string(),
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
            &ObjectClass::System(ref system) => system.to_proto(),
            &ObjectClass::Credential(ref credential) => credential.to_proto(),
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
            &ObjectClass::OrganizationalUnit(ref ou) => ou.objecthash(hasher),
            &ObjectClass::System(ref system) => system.objecthash(hasher),
            &ObjectClass::Credential(ref credential) => credential.objecthash(hasher),
        }
    }
}
