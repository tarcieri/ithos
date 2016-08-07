use std::io;

use buffoon::{self, Serialize, Deserialize, OutputStream, InputStream};

use algorithm::DigestAlgorithm;
use error::{Error, Result};
use log;

pub struct Root {
    pub logid: log::Id,
    pub digest_alg: DigestAlgorithm,
}

impl Root {
    pub fn new(logid: log::Id) -> Root {
        Root {
            logid: logid,
            digest_alg: DigestAlgorithm::SHA256
        }
    }

    pub fn to_proto(&self) -> Result<Vec<u8>> {
        buffoon::serialize(&self).map_err(|_| Error::Serialize)
    }
}

impl Serialize for Root {
    fn serialize<O: OutputStream>(&self, out: &mut O) -> io::Result<()> {
        try!(out.write(1, self.logid.as_ref()));
        try!(out.write(2, &self.digest_alg));
        Ok(())
    }
}

impl Deserialize for Root {
    fn deserialize<R: io::Read>(i: &mut InputStream<R>) -> io::Result<Root> {
        let mut logid_bytes: Option<Vec<u8>> = None;
        let mut digest_alg = None;

        while let Some(f) = try!(i.read_field()) {
            match f.tag() {
                1 => logid_bytes = Some(try!(f.read())),
                2 => digest_alg = Some(try!(f.read())),
                _ => try!(f.skip()),
            }
        }

        let logid = try!(log::Id::from_bytes(&required!(logid_bytes, "Root::logid"))
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "error parsing logid")));

        Ok(Root {
            logid: logid,
            digest_alg: required!(digest_alg, "Root::digest_alg")
        })
    }
}