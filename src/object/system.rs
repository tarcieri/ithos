use buffoon::{Serialize, Deserialize, OutputStream, InputStream};
use object::{AllowsChild, Object};
use objecthash::{self, ObjectHash, ObjectHasher};
use proto::{ToProto, FromProto};
use serde_json::builder::ObjectBuilder;
use std::io;

#[derive(Debug, Eq, PartialEq)]
pub struct SystemEntry {
    pub username: String,
}

impl SystemEntry {
    pub fn new(username: String) -> SystemEntry {
        SystemEntry { username: username }
    }

    pub fn build_json(&self, builder: ObjectBuilder) -> ObjectBuilder {
        builder.insert("username", &self.username)
    }
}

impl AllowsChild for SystemEntry {
    #[inline]
    fn allows_child(child: &Object) -> bool {
        match *child {
            Object::OrgUnit(_) => true,
            _ => false,
        }
    }
}

impl Serialize for SystemEntry {
    fn serialize<O: OutputStream>(&self, out: &mut O) -> io::Result<()> {
        try!(out.write(1, &self.username));
        Ok(())
    }
}

impl Deserialize for SystemEntry {
    fn deserialize<R: io::Read>(i: &mut InputStream<R>) -> io::Result<SystemEntry> {
        let mut username: Option<String> = None;

        while let Some(f) = try!(i.read_field()) {
            match f.tag() {
                1 => username = Some(try!(f.read())),
                _ => try!(f.skip()),
            }
        }

        Ok(SystemEntry { username: required!(username, "SystemObject::username") })
    }
}

impl ToProto for SystemEntry {}
impl FromProto for SystemEntry {}

impl ObjectHash for SystemEntry {
    #[inline]
    fn objecthash<H: ObjectHasher>(&self, hasher: &mut H) {
        objecthash_struct!(hasher, "username" => self.username);
    }
}
