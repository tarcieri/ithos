use buffoon::{self, Serialize, Deserialize};
use error::{Error, Result};
use std::marker::Sized;

pub trait ToProto
    where Self: Sized + Serialize
{
    fn to_proto(&self) -> Result<Vec<u8>> {
        buffoon::serialize(&self).map_err(|_| Error::serialize(None))
    }
}

pub trait FromProto
    where Self: Sized + Deserialize
{
    fn from_proto(bytes: &[u8]) -> Result<Self> {
        buffoon::deserialize(bytes).map_err(|_| Error::parse(None))
    }
}
