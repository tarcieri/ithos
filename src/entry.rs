use adapter::Adapter;
use error::{Error, Result};
use object::Object;
use object::credential::Credential;
use object::domain::Domain;
use object::org_unit::OrgUnit;
use object::root::Root;
use object::system::System;
use path::Path;
use protobuf::{self, Message};
use std::mem;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct Id(u64);

// Ids are 64-bit integers in host-native byte order
// LMDB has special optimizations for host-native integers as keys
impl Id {
    pub fn root() -> Id {
        Id(0)
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Id> {
        if bytes.len() != 8 {
            return Err(Error::parse(None));
        }

        let mut id = [0u8; 8];
        id.copy_from_slice(&bytes[0..8]);

        Ok(Id(unsafe { mem::transmute(id) }))
    }

    pub fn next(self) -> Id {
        Id(self.0 + 1)
    }
}

impl AsRef<[u8; 8]> for Id {
    #[inline]
    fn as_ref(&self) -> &[u8; 8] {
        unsafe { mem::transmute(&self.0) }
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Class {
    Root, // Root entry in the tree (ala LDAP root DSE)
    Domain, // Administrative Domain (ala DNS domain or Kerberos realm)
    OrgUnit, // Unit/department within an organization
    System, // System User (i.e. non-human account)
    Credential, // Encrypted access credential
}

impl Class {
    pub fn from_bytes(bytes: &[u8]) -> Result<Class> {
        if bytes.len() != 4 {
            return Err(Error::parse(None));
        }

        let mut id_bytes = [0u8; 4];
        id_bytes.copy_from_slice(&bytes[0..4]);

        let id: u32 = unsafe { mem::transmute(id_bytes) };

        let result = match id {
            0 => Class::Root,
            1 => Class::Domain,
            2 => Class::OrgUnit,
            3 => Class::System,
            4 => Class::Credential,
            _ => return Err(Error::parse(None)),
        };

        Ok(result)
    }

    pub fn from_object(object: &Object) -> Option<Class> {
        if object.has_root() {
            Some(Class::Root)
        } else if object.has_domain() {
            Some(Class::Domain)
        } else if object.has_org_unit() {
            Some(Class::OrgUnit)
        } else if object.has_system() {
            Some(Class::System)
        } else if object.has_credential() {
            Some(Class::Credential)
        } else {
            None
        }
    }

    // Are children of the given class allowed under this class?
    pub fn allows_child(&self, child: &Class) -> bool {
        match *self {
            Class::Root => {
                match *child {
                    Class::Domain => true,
                    _ => false,
                }
            }
            Class::Domain => {
                match *child {
                    Class::Domain | Class::OrgUnit => true,
                    _ => false,
                }
            }
            Class::OrgUnit => {
                match *child {
                    Class::OrgUnit | Class::System | Class::Credential => true,
                    _ => false,
                }
            }
            Class::System => {
                match *child {
                    Class::OrgUnit => true,
                    _ => false,
                }
            }
            Class::Credential => false,
        }
    }

    #[inline]
    pub fn as_bytes(&self) -> [u8; 4] {
        let id = *self as u32;
        unsafe { mem::transmute(id) }
    }
}

impl ToString for Class {
    fn to_string(&self) -> String {
        match *self {
            Class::Root => "root".to_string(),
            Class::Domain => "domain".to_string(),
            Class::OrgUnit => "org_unit".to_string(),
            Class::System => "system".to_string(),
            Class::Credential => "credential".to_string(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Entry {
    Root(Root), // Root entry in the tree (ala LDAP root DSE)
    Domain(Domain), // Administrative Domain (ala DNS domain or Kerberos realm)
    OrgUnit(OrgUnit), // Unit/department within an organization
    System(System), // System User (i.e. non-human account)
    Credential(Credential), // Encrypted access credential
}

impl Entry {
    pub fn find<'a, A>(adapter: &'a A, path: &Path) -> Result<Entry>
        where A: Adapter<'a>
    {
        let txn = try!(adapter.ro_transaction());
        let direntry = try!(adapter.find_direntry(&txn, path));
        let entry = try!(adapter.find_entry(&txn, &direntry.id));

        Ok(try!(entry.deserialize()))
    }

    pub fn from_object(object: &mut Object) -> Option<Entry> {
        if object.has_root() {
            Some(Entry::Root(object.take_root()))
        } else if object.has_domain() {
            Some(Entry::Domain(object.take_domain()))
        } else if object.has_org_unit() {
            Some(Entry::OrgUnit(object.take_org_unit()))
        } else if object.has_system() {
            Some(Entry::System(object.take_system()))
        } else if object.has_credential() {
            Some(Entry::Credential(object.take_credential()))
        } else {
            None
        }
    }

    pub fn serialize(&self) -> Result<Vec<u8>> {
        let result = match *self {
            Entry::Root(ref entry) => entry.write_to_bytes(),
            Entry::Domain(ref entry) => entry.write_to_bytes(),
            Entry::OrgUnit(ref entry) => entry.write_to_bytes(),
            Entry::System(ref entry) => entry.write_to_bytes(),
            Entry::Credential(ref entry) => entry.write_to_bytes(),
        };

        Ok(try!(result))
    }
}

pub struct SerializedEntry<'a> {
    pub id: Id,
    pub class: Class,
    pub data: &'a [u8],
}

impl<'a> SerializedEntry<'a> {
    pub fn from_bytes(id: Id, bytes: &[u8]) -> Result<SerializedEntry> {
        if bytes.len() < 4 {
            return Err(Error::parse(None));
        }

        Ok(SerializedEntry {
            id: id,
            class: try!(Class::from_bytes(&bytes[0..4])),
            data: &bytes[4..],
        })
    }

    pub fn deserialize(&self) -> Result<Entry> {
        let result = match self.class {
            Class::Root => Entry::Root(try!(protobuf::parse_from_bytes::<Root>(self.data))),
            Class::Domain => Entry::Domain(try!(protobuf::parse_from_bytes::<Domain>(self.data))),
            Class::OrgUnit => {
                Entry::OrgUnit(try!(protobuf::parse_from_bytes::<OrgUnit>(self.data)))
            }
            Class::System => Entry::System(try!(protobuf::parse_from_bytes::<System>(self.data))),
            Class::Credential => {
                Entry::Credential(try!(protobuf::parse_from_bytes::<Credential>(self.data)))
            }
        };

        Ok(result)
    }
}
