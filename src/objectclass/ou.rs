use std::io;

use buffoon::{Serialize, Deserialize, OutputStream, InputStream};
use serde_json::builder::ObjectBuilder;

use proto::{ToProto, FromProto};
use objectclass::{AllowsChild, ObjectClass};
use objecthash::{self, ObjectHash, ObjectHasher};

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct OrganizationalUnitObject {
    pub description: Option<String>,
}

impl OrganizationalUnitObject {
    pub fn new(description: Option<String>) -> OrganizationalUnitObject {
        OrganizationalUnitObject { description: description }
    }

    pub fn build_json(&self, builder: ObjectBuilder) -> ObjectBuilder {
        builder.insert("description", &self.description)
    }
}

impl AllowsChild for OrganizationalUnitObject {
    #[inline]
    fn allows_child(&self, child: &ObjectClass) -> bool {
        match *child {
            ObjectClass::OrganizationalUnit(_) => true,
            ObjectClass::System(_) => true,
            ObjectClass::Credential(_) => true,
            _ => false,
        }
    }
}

impl Serialize for OrganizationalUnitObject {
    fn serialize<O: OutputStream>(&self, out: &mut O) -> io::Result<()> {
        match self.description {
            Some(ref description) => try!(out.write(1, &description)),
            None => (),
        }

        Ok(())
    }
}

impl Deserialize for OrganizationalUnitObject {
    fn deserialize<R: io::Read>(i: &mut InputStream<R>) -> io::Result<OrganizationalUnitObject> {
        let mut description: Option<String> = None;

        while let Some(f) = try!(i.read_field()) {
            match f.tag() {
                1 => description = Some(try!(f.read())),
                _ => try!(f.skip()),
            }
        }

        Ok(OrganizationalUnitObject { description: description })
    }
}

impl ToProto for OrganizationalUnitObject {}
impl FromProto for OrganizationalUnitObject {}

impl ObjectHash for OrganizationalUnitObject {
    #[inline]
    fn objecthash<H: ObjectHasher>(&self, hasher: &mut H) {
        match self.description {
            Some(ref desc) => {
                objecthash_struct!(
                    hasher,
                    "description" => *desc
                )
            }
            None => (),
        }
    }
}
