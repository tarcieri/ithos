use std::io;

use buffoon::{self, Serialize, Deserialize, OutputStream, InputStream};

use error::{Error, Result};
use objectclass::ObjectClass;

pub struct Domain {
    pub description: Option<String>,
}

impl Domain {
    pub fn new(description: Option<String>) -> Domain {
        Domain {
            description: description
        }
    }
}

impl ObjectClass for Domain {
    fn to_proto(&self) -> Result<Vec<u8>> {
        buffoon::serialize(&self).map_err(|_| Error::Serialize)
    }
}

impl Serialize for Domain {
    fn serialize<O: OutputStream>(&self, out: &mut O) -> io::Result<()> {
        match self.description {
            Some(ref description) => try!(out.write(1, &description)),
            None => ()
        }

        Ok(())
    }
}

impl Deserialize for Domain {
    fn deserialize<R: io::Read>(i: &mut InputStream<R>) -> io::Result<Domain> {
        let mut description: Option<String> = None;

        while let Some(f) = try!(i.read_field()) {
            match f.tag() {
                1 => description = Some(try!(f.read())),
                _ => try!(f.skip()),
            }
        }

        Ok(Domain {
            description: description
        })
    }
}