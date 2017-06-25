//! entry.rs: Entries within a directory tree
//!
//! Types for working with serialized and deserialized entries in the directory tree
//!

use adapter::Adapter;
use byteorder::{ByteOrder, NativeEndian};
use errors::*;
use id::EntryId;
use object::Object;
use object::credential::Credential;
use object::domain::Domain;
use object::org_unit::OrgUnit;
use object::root::Root;
use object::system::System;
use path::Path;
use protobuf::{self, Message};

/// Number of bytes of space in a serialized entry header
pub const HEADER_SIZE: usize = 4;

/// Entry type (ala LDAP objectclass)
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Class {
    /// Root entry in the tree (ala LDAP root DSE)
    Root,

    /// Administrative Domain (ala DNS domain or Kerberos realm)
    Domain,

    /// Unit/department within an organization
    OrgUnit,

    /// System User (i.e. non-human account)
    System,

    /// Encrypted access credential
    Credential,
}

impl Class {
    /// Deserialize an entry ID from its byte representation (host-native endianness)
    pub fn from_bytes(bytes: &[u8]) -> Result<Class> {
        if bytes.len() != HEADER_SIZE {
            let msg = format!("entry header too small: {}", bytes.len());
            return Err(ErrorKind::ParseFailure(msg).into());
        }

        let result = match NativeEndian::read_u32(bytes) {
            0 => Class::Root,
            1 => Class::Domain,
            2 => Class::OrgUnit,
            3 => Class::System,
            4 => Class::Credential,
            other => {
                let msg = format!("bad entry type: {}", other);
                return Err(ErrorKind::ParseFailure(msg).into());
            }
        };

        Ok(result)
    }

    /// Obtain the class associated with a given object
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

    /// Are children of the given class allowed under this class?
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

    /// Serialize class as its byte representation (host-native endianness)
    #[inline]
    pub fn as_bytes(&self) -> [u8; 4] {
        let mut result = [0u8; 4];
        NativeEndian::write_u32(&mut result, *self as u32);
        result
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

/// Sum type representing deserialized entries
#[derive(Debug, PartialEq)]
pub enum Entry {
    /// Root entry in the tree (ala LDAP root DSE)
    Root(Root),

    /// Administrative Domain (ala DNS domain or Kerberos realm)
    Domain(Domain),

    /// Unit/department within an organization
    OrgUnit(OrgUnit),

    /// System User (i.e. non-human account)
    System(System),

    /// Encrypted access credential
    Credential(Credential),
}

impl Entry {
    /// Find an entry located at the given path
    pub fn find<'a, A>(adapter: &'a A, path: &Path) -> Result<Entry>
    where
        A: Adapter<'a>,
    {
        let txn = adapter.ro_transaction()?;
        let direntry = adapter.find_direntry(&txn, path)?;
        let entry = adapter.find_entry(&txn, &direntry.id)?;

        Ok(entry.deserialize()?)
    }

    /// Convert an object to the `Entry` sum type
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

    /// Serialize an entry in its byte representation
    pub fn serialize(&self) -> Result<Vec<u8>> {
        let result = match *self {
            Entry::Root(ref entry) => entry.write_to_bytes()?,
            Entry::Domain(ref entry) => entry.write_to_bytes()?,
            Entry::OrgUnit(ref entry) => entry.write_to_bytes()?,
            Entry::System(ref entry) => entry.write_to_bytes()?,
            Entry::Credential(ref entry) => entry.write_to_bytes()?,
        };

        Ok(result)
    }
}

/// Raw entry serialized as bytes (i.e. reference type)
pub struct SerializedEntry<'a> {
    /// Host-native integer identifier for this entry
    pub id: EntryId,

    /// Class associated with this entry
    pub class: Class,

    /// Raw serialized entry (Protobuf)
    pub data: &'a [u8],
}

impl<'a> SerializedEntry<'a> {
    /// Parse entry header and determine its type
    pub fn from_bytes(id: EntryId, bytes: &[u8]) -> Result<SerializedEntry> {
        if bytes.len() < HEADER_SIZE {
            let msg = format!("entry header too small: {}", bytes.len());
            return Err(ErrorKind::ParseFailure(msg).into());
        }

        Ok(SerializedEntry {
            id: id,
            class: Class::from_bytes(&bytes[0..HEADER_SIZE])?,
            data: &bytes[HEADER_SIZE..],
        })
    }

    /// Deserialize an entry into the (owned) `Entry` sum type
    pub fn deserialize(&self) -> Result<Entry> {
        Ok(match self.class {
            Class::Root => Entry::Root(protobuf::parse_from_bytes::<Root>(self.data)?),
            Class::Domain => Entry::Domain(protobuf::parse_from_bytes::<Domain>(self.data)?),
            Class::OrgUnit => Entry::OrgUnit(protobuf::parse_from_bytes::<OrgUnit>(self.data)?),
            Class::System => Entry::System(protobuf::parse_from_bytes::<System>(self.data)?),
            Class::Credential => {
                Entry::Credential(protobuf::parse_from_bytes::<Credential>(self.data)?)
            }
        })
    }
}
