pub mod credential;
pub mod domain;
pub mod ou;
pub mod root;
pub mod system;

use std::io;
use std::mem;
use std::string::ToString;

use buffoon::{Serialize, OutputStream};
use serde_json::builder::ObjectBuilder;
use objecthash::{ObjectHash, ObjectHasher};

use entry::Entry;
use error::{Error, Result};
use proto::{ToProto, FromProto};

use self::credential::CredentialEntry;
use self::domain::DomainEntry;
use self::ou::OrganizationalUnitEntry;
use self::root::RootEntry;
use self::system::SystemEntry;

// Object nesting constraints
pub trait AllowsChild {
    fn allows_child(child: &Object) -> bool;
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Class {
    Root, // Root entry in the tree (ala LDAP root DSE)
    Domain, // Administrative Domain (ala DNS domain or Kerberos realm)
    OrganizationalUnit, // Unit/department within an organization
    System, // System User (i.e. non-human account)
    Credential, // Encrypted access credential
}

impl Class {
    pub fn from_bytes(bytes: &[u8]) -> Result<Class> {
        if bytes.len() != 4 {
            return Err(Error::Parse);
        }

        let mut id_bytes = [0u8; 4];
        id_bytes.copy_from_slice(&bytes[0..4]);

        let id: u32 = unsafe { mem::transmute(id_bytes) };

        let result = match id {
            0 => Class::Root,
            1 => Class::Domain,
            2 => Class::OrganizationalUnit,
            3 => Class::System,
            4 => Class::Credential,
            _ => return Err(Error::Parse),
        };

        Ok(result)
    }


    #[inline]
    pub fn allows_child(&self, child: &Object) -> bool {
        match *self {
            Class::Root => RootEntry::allows_child(child),
            Class::Domain => DomainEntry::allows_child(child),
            Class::OrganizationalUnit => OrganizationalUnitEntry::allows_child(child),
            Class::System => SystemEntry::allows_child(child),
            Class::Credential => CredentialEntry::allows_child(child),
        }
    }

    #[inline]
    pub fn as_bytes(&self) -> [u8; 4] {
        let id = *self as u32 + 1;
        unsafe { mem::transmute(id) }
    }
}

impl ToString for Class {
    fn to_string(&self) -> String {
        match *self {
            Class::Root => "ROOT".to_string(),
            Class::Domain => "DOMAIN".to_string(),
            Class::OrganizationalUnit => "ORGANIZATIONAL_UNIT".to_string(),
            Class::System => "SYSTEM".to_string(),
            Class::Credential => "CREDENTIAL".to_string(),
        }
    }
}

impl Serialize for Class {
    fn serialize<O: OutputStream>(&self, _: &mut O) -> io::Result<()> {
        unimplemented!();
    }

    fn serialize_nested<O: OutputStream>(&self, field: u32, out: &mut O) -> io::Result<()> {
        out.write_varint(field, *self as u32 + 1)
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum Object {
    Root(RootEntry), // Root entry in the tree (ala LDAP root DSE)
    Domain(DomainEntry), // Administrative Domain (ala DNS domain or Kerberos realm)
    OrganizationalUnit(OrganizationalUnitEntry), // Unit/department within an organization
    System(SystemEntry), // System User (i.e. non-human account)
    Credential(CredentialEntry), // Encrypted access credential
}

impl Object {
    #[inline]
    pub fn class(&self) -> Class {
        match *self {
            Object::Root(_) => Class::Root,
            Object::Domain(_) => Class::Domain,
            Object::OrganizationalUnit(_) => Class::OrganizationalUnit,
            Object::System(_) => Class::System,
            Object::Credential(_) => Class::Credential,
        }
    }

    #[allow(dead_code)]
    pub fn from_entry(entry: Entry) -> Result<Object> {
        let result = match entry.class {
            Class::Root => Object::Root(try!(RootEntry::from_proto(entry.data))),
            Class::Domain => Object::Domain(try!(DomainEntry::from_proto(entry.data))),
            Class::OrganizationalUnit => {
                Object::OrganizationalUnit(try!(OrganizationalUnitEntry::from_proto(entry.data)))
            }
            Class::System => Object::System(try!(SystemEntry::from_proto(entry.data))),
            Class::Credential => Object::Credential(try!(CredentialEntry::from_proto(entry.data))),
        };

        Ok(result)
    }

    pub fn build_json(&self, builder: ObjectBuilder) -> ObjectBuilder {
        builder.insert("class", self.class().to_string())
            .insert_object("value", |b| match *self {
                Object::Root(ref root) => root.build_json(b),
                Object::Domain(ref domain) => domain.build_json(b),
                Object::OrganizationalUnit(ref ou) => ou.build_json(b),
                Object::System(ref system) => system.build_json(b),
                Object::Credential(ref credential) => credential.build_json(b),
            })
    }
}

impl Serialize for Object {
    fn serialize<O: OutputStream>(&self, out: &mut O) -> io::Result<()> {
        try!(out.write(1, &self.class()));

        let object_proto = match self {
            &Object::Root(ref root) => root.to_proto(),
            &Object::Domain(ref domain) => domain.to_proto(),
            &Object::OrganizationalUnit(ref ou) => ou.to_proto(),
            &Object::System(ref system) => system.to_proto(),
            &Object::Credential(ref credential) => credential.to_proto(),
        };

        if !object_proto.is_ok() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("couldn't serialize {type}", type = self.class().to_string())
            ));
        }

        try!(out.write(2, &object_proto.unwrap()));

        Ok(())
    }
}

impl ToProto for Object {}

impl ObjectHash for Object {
    fn objecthash<H: ObjectHasher>(&self, hasher: &mut H) {
        match self {
            &Object::Root(ref root) => root.objecthash(hasher),
            &Object::Domain(ref domain) => domain.objecthash(hasher),
            &Object::OrganizationalUnit(ref ou) => ou.objecthash(hasher),
            &Object::System(ref system) => system.objecthash(hasher),
            &Object::Credential(ref credential) => credential.objecthash(hasher),
        }
    }
}
