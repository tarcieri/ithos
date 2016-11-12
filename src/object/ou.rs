use std::io;

use buffoon::{Serialize, Deserialize, OutputStream, InputStream};
use serde_json::builder::ObjectBuilder;

use proto::{ToProto, FromProto};
use object::{AllowsChild, Object};
use objecthash::{self, ObjectHash, ObjectHasher};

#[derive(Debug, Eq, PartialEq)]
pub struct OrgUnitEntry {
    pub description: Option<String>,
}

impl OrgUnitEntry {
    pub fn new(description: Option<String>) -> OrgUnitEntry {
        OrgUnitEntry { description: description }
    }

    pub fn build_json(&self, builder: ObjectBuilder) -> ObjectBuilder {
        builder.insert("description", &self.description)
    }
}

impl AllowsChild for OrgUnitEntry {
    #[inline]
    fn allows_child(child: &Object) -> bool {
        match *child {
            Object::OrgUnit(_) => true,
            Object::System(_) => true,
            Object::Credential(_) => true,
            _ => false,
        }
    }
}

impl Serialize for OrgUnitEntry {
    fn serialize<O: OutputStream>(&self, out: &mut O) -> io::Result<()> {
        match self.description {
            Some(ref description) => try!(out.write(1, &description)),
            None => (),
        }

        Ok(())
    }
}

impl Deserialize for OrgUnitEntry {
    fn deserialize<R: io::Read>(i: &mut InputStream<R>) -> io::Result<OrgUnitEntry> {
        let mut description: Option<String> = None;

        while let Some(f) = try!(i.read_field()) {
            match f.tag() {
                1 => description = Some(try!(f.read())),
                _ => try!(f.skip()),
            }
        }

        Ok(OrgUnitEntry { description: description })
    }
}

impl ToProto for OrgUnitEntry {}
impl FromProto for OrgUnitEntry {}

impl ObjectHash for OrgUnitEntry {
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