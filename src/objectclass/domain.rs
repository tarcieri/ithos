use std::io;

use buffoon::{Serialize, Deserialize, OutputStream, InputStream};
use serde_json::builder::ObjectBuilder;

use proto::{ToProto, FromProto};
use objectclass::{AllowsChild, ObjectClass};
use objecthash::{self, ObjectHash, ObjectHasher};

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct DomainObject {
    pub description: Option<String>,
}

impl DomainObject {
    pub fn new(description: Option<String>) -> DomainObject {
        DomainObject { description: description }
    }

    pub fn build_json(&self, builder: ObjectBuilder) -> ObjectBuilder {
        builder.insert("description", &self.description)
    }
}

impl AllowsChild for DomainObject {
    #[inline]
    fn allows_child(&self, child: &ObjectClass) -> bool {
        match *child {
            ObjectClass::Domain(_) => true,
            ObjectClass::OrganizationalUnit(_) => true,
            _ => false,
        }
    }
}

impl Serialize for DomainObject {
    fn serialize<O: OutputStream>(&self, out: &mut O) -> io::Result<()> {
        match self.description {
            Some(ref description) => try!(out.write(1, &description)),
            None => (),
        }

        Ok(())
    }
}

impl Deserialize for DomainObject {
    fn deserialize<R: io::Read>(i: &mut InputStream<R>) -> io::Result<DomainObject> {
        let mut description: Option<String> = None;

        while let Some(f) = try!(i.read_field()) {
            match f.tag() {
                1 => description = Some(try!(f.read())),
                _ => try!(f.skip()),
            }
        }

        Ok(DomainObject { description: description })
    }
}

impl ToProto for DomainObject {}
impl FromProto for DomainObject {}

impl ObjectHash for DomainObject {
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
