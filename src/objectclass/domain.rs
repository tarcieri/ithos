use std::io;

use buffoon::{Serialize, Deserialize, OutputStream, InputStream};
use ring::digest;

use proto::ToProto;
use objecthash::{ObjectHash, DIGEST_ALG};

#[derive(Debug, Eq, PartialEq)]
pub struct DomainObject {
    pub description: Option<String>,
}

impl DomainObject {
    pub fn new(description: Option<String>) -> DomainObject {
        DomainObject { description: description }
    }
}

impl ToProto for DomainObject {}

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

impl ObjectHash for DomainObject {
    fn objecthash(&self) -> digest::Digest {
        let mut ctx = digest::Context::new(&DIGEST_ALG);

        // objecthash qualifier for dictionaries
        ctx.update(b"d");

        match self.description {
            Some(ref desc) => {
                ctx.update("description".objecthash().as_ref());
                ctx.update(desc.objecthash().as_ref());
            }
            None => (),
        }

        ctx.finish()
    }
}
