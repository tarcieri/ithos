use algorithm::DigestAlgorithm;
use buffoon::{Serialize, Deserialize, OutputStream, InputStream};
use object::{AllowsChild, Object};
use objecthash::{self, ObjectHash, ObjectHasher};
use proto::{ToProto, FromProto};
use std::io;

#[derive(Debug, Eq, PartialEq)]
pub struct RootEntry {
    pub digest_alg: DigestAlgorithm,
}

impl RootEntry {
    pub fn new(digest_alg: DigestAlgorithm) -> RootEntry {
        RootEntry { digest_alg: digest_alg }
    }
}

impl AllowsChild for RootEntry {
    #[inline]
    fn allows_child(child: &Object) -> bool {
        match *child {
            Object::Domain(_) => true,
            _ => false,
        }
    }
}

impl ToProto for RootEntry {}
impl FromProto for RootEntry {}

impl Serialize for RootEntry {
    fn serialize<O: OutputStream>(&self, out: &mut O) -> io::Result<()> {
        try!(out.write(1, &self.digest_alg));
        Ok(())
    }
}

impl Deserialize for RootEntry {
    fn deserialize<R: io::Read>(i: &mut InputStream<R>) -> io::Result<RootEntry> {
        let mut digest_alg = None;

        while let Some(f) = try!(i.read_field()) {
            match f.tag() {
                1 => digest_alg = Some(try!(f.read())),
                _ => try!(f.skip()),
            }
        }

        Ok(RootEntry { digest_alg: required!(digest_alg, "Root::digest_alg") })
    }
}

impl ObjectHash for RootEntry {
    #[inline]
    fn objecthash<H: ObjectHasher>(&self, hasher: &mut H) {
        objecthash_struct!(
            hasher,
            "digest_alg" => self.digest_alg
        )
    }
}
