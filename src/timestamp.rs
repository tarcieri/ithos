use buffoon::{self, Field, InputStream, OutputStream};
use objecthash::{ObjectHash, ObjectHasher};
use serde;
use std::io;
use time;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct Timestamp(u64);

impl Timestamp {
    pub fn at(secs: u64) -> Timestamp {
        Timestamp(secs)
    }

    pub fn now() -> Timestamp {
        Timestamp(time::now_utc().to_timespec().sec as u64)
    }

    pub fn extend(&self, seconds: u64) -> Timestamp {
        Timestamp(self.0 + seconds)
    }
}

impl ObjectHash for Timestamp {
    fn objecthash<H: ObjectHasher>(&self, hasher: &mut H) {
        self.0.objecthash(hasher);
    }
}

impl serde::ser::Serialize for Timestamp {
    #[inline]
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
        where S: serde::ser::Serializer
    {
        serializer.serialize_u64(self.0)
    }
}

impl buffoon::Serialize for Timestamp {
    fn serialize<O: OutputStream>(&self, _: &mut O) -> io::Result<()> {
        unimplemented!();
    }

    #[inline]
    fn serialize_nested<O: OutputStream>(&self, field: u32, out: &mut O) -> io::Result<()> {
        out.write_varint(field, self.0)
    }
}

impl buffoon::Deserialize for Timestamp {
    fn deserialize<R: io::Read>(_: &mut InputStream<R>) -> io::Result<Self> {
        unimplemented!();
    }

    #[inline]
    fn deserialize_nested<R: io::Read>(field: Field<R>) -> io::Result<Timestamp> {
        Ok(Timestamp(try!(field.read_varint())))
    }
}
