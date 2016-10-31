use std::io;

use buffoon::{Serialize, Deserialize, OutputStream, InputStream};
use serde_json::builder::ObjectBuilder;
use rustc_serialize::base64::{self, ToBase64};

use algorithm::DigestAlgorithm;
use log;
use proto::{ToProto, FromProto};
use object::{AllowsChild, Object};
use objecthash::{self, ObjectHash, ObjectHasher};

#[derive(Debug, Eq, PartialEq)]
pub struct RootEntry {
    pub logid: log::Id,
    pub digest_alg: DigestAlgorithm,
}

impl RootEntry {
    pub fn new(logid: log::Id) -> RootEntry {
        RootEntry {
            logid: logid,
            digest_alg: DigestAlgorithm::Sha256,
        }
    }

    pub fn build_json(&self, builder: ObjectBuilder) -> ObjectBuilder {
        builder.insert("logid", self.logid.as_ref().to_base64(base64::URL_SAFE))
            .insert("digest_alg", self.digest_alg.to_string())
    }
}

impl AllowsChild for RootEntry {
    #[inline]
    fn allows_child(child: &Object) -> bool {
        match *child {
            Object::OrgUnit(_) => true,
            Object::Domain(_) => true,
            _ => false,
        }
    }
}

impl ToProto for RootEntry {}
impl FromProto for RootEntry {}

impl Serialize for RootEntry {
    fn serialize<O: OutputStream>(&self, out: &mut O) -> io::Result<()> {
        try!(out.write(1, self.logid.as_ref()));
        try!(out.write(2, &self.digest_alg));
        Ok(())
    }
}

impl Deserialize for RootEntry {
    fn deserialize<R: io::Read>(i: &mut InputStream<R>) -> io::Result<RootEntry> {
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

        Ok(RootEntry {
            logid: logid,
            digest_alg: required!(digest_alg, "Root::digest_alg"),
        })
    }
}

impl ObjectHash for RootEntry {
    #[inline]
    fn objecthash<H: ObjectHasher>(&self, hasher: &mut H) {
        objecthash_struct!(
            hasher,
            "logid" => self.logid,
            "digest_alg" => self.digest_alg
        )
    }
}
