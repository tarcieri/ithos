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

use entry::{self, Entry};
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
pub struct ClassId(u32);

impl ClassId {
    pub fn from_bytes(bytes: &[u8]) -> Result<ClassId> {
        if bytes.len() != 4 {
            return Err(Error::Parse);
        }

        let mut id = [0u8; 4];
        id.copy_from_slice(&bytes[0..4]);

        Ok(ClassId(unsafe { mem::transmute(id) }))
    }
}

impl AsRef<[u8; 4]> for ClassId {
    #[inline(always)]
    fn as_ref(&self) -> &[u8; 4] {
        unsafe { mem::transmute(&self.0) }
    }
}

impl Serialize for ClassId {
    fn serialize<O: OutputStream>(&self, _: &mut O) -> io::Result<()> {
        unimplemented!();
    }

    fn serialize_nested<O: OutputStream>(&self, field: u32, out: &mut O) -> io::Result<()> {
        out.write_varint(field, self.0)
    }
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
    #[inline]
    pub fn id(&self) -> ClassId {
        ClassId(*self as u32 + 1)
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

#[derive(Debug, Eq, PartialEq, Clone)]
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

    pub fn from_entry(entry: Entry) -> Result<Object> {
        let ClassId(class_id) = entry.class_id;

        let result = match class_id {
            1 => Object::Root(try!(RootEntry::from_proto(entry.data))),
            2 => Object::Domain(try!(DomainEntry::from_proto(entry.data))),
            3 => Object::OrganizationalUnit(try!(OrganizationalUnitEntry::from_proto(entry.data))),
            4 => Object::System(try!(SystemEntry::from_proto(entry.data))),
            5 => Object::Credential(try!(CredentialEntry::from_proto(entry.data))),
            _ => return Err(Error::Parse),
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
        try!(out.write(1, &self.class().id()));

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
