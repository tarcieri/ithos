use std::io;

use buffoon::{Serialize, Deserialize, OutputStream, InputStream};

use algorithm::DigestAlgorithm;
use log;
use proto::ToProto;
use objecthash::{self, ObjectHash, ObjectHasher};

#[derive(Debug, Eq, PartialEq)]
pub struct RootObject {
    pub logid: log::Id,
    pub digest_alg: DigestAlgorithm,
}

impl RootObject {
    pub fn new(logid: log::Id) -> RootObject {
        RootObject {
            logid: logid,
            digest_alg: DigestAlgorithm::Sha256,
        }
    }
}

impl ToProto for RootObject {}

impl Serialize for RootObject {
    fn serialize<O: OutputStream>(&self, out: &mut O) -> io::Result<()> {
        try!(out.write(1, self.logid.as_ref()));
        try!(out.write(2, &self.digest_alg));
        Ok(())
    }
}

impl Deserialize for RootObject {
    fn deserialize<R: io::Read>(i: &mut InputStream<R>) -> io::Result<RootObject> {
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

        Ok(RootObject {
            logid: logid,
            digest_alg: required!(digest_alg, "Root::digest_alg"),
        })
    }
}

impl ObjectHash for RootObject {
    #[inline]
    fn objecthash<H: ObjectHasher>(&self, hasher: &mut H) {
        objecthash_struct!(
            hasher,
            "logid" => self.logid,
            "digest_alg" => self.digest_alg
        )
    }
}
