use ring::rand::SecureRandom;

use error::{Error, Result};

pub const ID_SIZE: usize = 16;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct Id([u8; ID_SIZE]);

impl Id {
    pub fn generate(rng: &SecureRandom) -> Result<Id> {
        let mut logid = [0u8; ID_SIZE];
        try!(rng.fill(&mut logid).map_err(|_| Error::Rng));
        Ok(Id(logid))
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Id> {
        if bytes.len() != ID_SIZE {
            return Err(Error::Parse);
        }

        let mut id = [0u8; ID_SIZE];
        id.copy_from_slice(&bytes[0..ID_SIZE]);

        Ok(Id(id))
    }
}

impl AsRef<[u8]> for Id {
    #[inline(always)]
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}