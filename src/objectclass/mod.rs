pub mod root;

use std::io;
use std::string::ToString;
use ring::digest::Digest;

use buffoon::{Serialize, Deserialize, OutputStream, InputStream, Field};

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

impl Serialize for ObjectClass {
    fn serialize<O: OutputStream>(&self, _: &mut O) -> io::Result<()> {
        unimplemented!();
    }

    fn serialize_nested<O: OutputStream>(&self, field: u32, out: &mut O) -> io::Result<()> {
        out.write_varint(field, *self as u32 + 1)
    }
}

impl Deserialize for ObjectClass {
    fn deserialize<R: io::Read>(_: &mut InputStream<R>) -> io::Result<ObjectClass> {
        unimplemented!();
    }

    fn deserialize_nested<R: io::Read>(field: Field<R>) -> io::Result<ObjectClass> {
        match try!(u32::deserialize_nested(field)) {
            1 => Ok(ObjectClass::Root),
            2 => Ok(ObjectClass::Domain),
            3 => Ok(ObjectClass::Ou),
            4 => Ok(ObjectClass::Credential),
            5 => Ok(ObjectClass::System),
            6 => Ok(ObjectClass::Host),
            _ => Err(io::Error::new(io::ErrorKind::InvalidInput, "unknown ObjectClass")),
        }
    }
}