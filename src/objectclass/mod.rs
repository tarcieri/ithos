pub mod root;

use std::io;
use std::string::ToString;
use ring::digest::Digest;

use buffoon::{Serialize, Deserialize, OutputStream, InputStream, Field};

use error::Result;
use objecthash::ObjectHash;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Type {
    Root, // Root DSE
    Domain, // ala DNS domain or Kerberos realm
    Ou, // Organizational unit
    Credential, // Encrypted access credential
    System, // System User (i.e. non-human account)
    Host, // an individual server
}

pub trait ObjectClass {
    fn to_proto(&self) -> Result<Vec<u8>>;
}

impl ToString for Type {
    fn to_string(&self) -> String {
        match *self {
            Type::Root => "root".to_string(),
            Type::Domain => "domain".to_string(),
            Type::Ou => "ou".to_string(),
            Type::Credential => "credential".to_string(),
            Type::System => "system".to_string(),
            Type::Host => "host".to_string(),
        }
    }
}

impl ObjectHash for Type {
    fn objecthash(&self) -> Digest {
        self.to_string().objecthash()
    }
}

impl Serialize for Type {
    fn serialize<O: OutputStream>(&self, _: &mut O) -> io::Result<()> {
        unimplemented!();
    }

    fn serialize_nested<O: OutputStream>(&self, field: u32, out: &mut O) -> io::Result<()> {
        out.write_varint(field, *self as u32 + 1)
    }
}

impl Deserialize for Type {
    fn deserialize<R: io::Read>(_: &mut InputStream<R>) -> io::Result<Type> {
        unimplemented!();
    }

    fn deserialize_nested<R: io::Read>(field: Field<R>) -> io::Result<Type> {
        match try!(u32::deserialize_nested(field)) {
            1 => Ok(Type::Root),
            2 => Ok(Type::Domain),
            3 => Ok(Type::Ou),
            4 => Ok(Type::Credential),
            5 => Ok(Type::System),
            6 => Ok(Type::Host),
            _ => Err(io::Error::new(io::ErrorKind::InvalidInput, "unknown Type")),
        }
    }
}
