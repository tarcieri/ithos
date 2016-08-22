use std::io;

use buffoon::{Serialize, Deserialize, OutputStream, InputStream};
use serde_json::builder::ObjectBuilder;

use proto::{ToProto, FromProto};
use objectclass::{AllowsChild, ObjectClass};
use objecthash::{self, ObjectHash, ObjectHasher};

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct SystemObject {
    pub username: String,
}

impl SystemObject {
    pub fn new(username: String) -> SystemObject {
        SystemObject { username: username }
    }

    pub fn build_json(&self, builder: ObjectBuilder) -> ObjectBuilder {
        builder.insert("username", &self.username)
    }
}

impl AllowsChild for SystemObject {
    #[inline]
    fn allows_child(&self, child: &ObjectClass) -> bool {
        match *child {
            ObjectClass::OrganizationalUnit(_) => true,
            _ => false,
        }
    }
}

impl Serialize for SystemObject {
    fn serialize<O: OutputStream>(&self, out: &mut O) -> io::Result<()> {
        try!(out.write(1, &self.username));
        Ok(())
    }
}

impl Deserialize for SystemObject {
    fn deserialize<R: io::Read>(i: &mut InputStream<R>) -> io::Result<SystemObject> {
        let mut username: Option<String> = None;

        while let Some(f) = try!(i.read_field()) {
            match f.tag() {
                1 => username = Some(try!(f.read())),
                _ => try!(f.skip()),
            }
        }

        Ok(SystemObject { username: required!(username, "SystemObject::username") })
    }
}

impl ToProto for SystemObject {}
impl FromProto for SystemObject {}

impl ObjectHash for SystemObject {
    #[inline]
    fn objecthash<H: ObjectHasher>(&self, hasher: &mut H) {
        objecthash_struct!(hasher, "username" => self.username);
    }
}
