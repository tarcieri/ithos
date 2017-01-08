use buffoon::{OutputStream, Serialize};
use objecthash::{self, ObjectHash, ObjectHasher};
use proto::ToProto;
use signature::Signature;
use std::io;

#[derive(Debug, Eq, PartialEq)]
pub struct Witness {
    pub signatures: Vec<Signature>,
}

impl ToProto for Witness {}

impl Serialize for Witness {
    fn serialize<O: OutputStream>(&self, out: &mut O) -> io::Result<()> {
        try!(out.write_repeated(1, &self.signatures));
        Ok(())
    }
}

impl ObjectHash for Witness {
    #[inline]
    fn objecthash<H: ObjectHasher>(&self, hasher: &mut H) {
        objecthash_struct!(hasher, "signatures" => self.signatures);
    }
}
