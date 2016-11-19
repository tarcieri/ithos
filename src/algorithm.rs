

use buffoon::{Serialize, Deserialize, OutputStream, InputStream, Field};

use objecthash::{ObjectHash, ObjectHasher};
use std::io;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum DigestAlgorithm {
    Sha256,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum EncryptionAlgorithm {
    Aes256Gcm,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum SignatureAlgorithm {
    Ed25519,
}

// TODO: Support more than one algorithm type per enum
macro_rules! impl_algorithm (($algorithm:ident, $only:expr, $string:expr) => (
    impl $algorithm {
        #[allow(dead_code)]
        pub fn id(&self) -> u32 {
            *self as u32 + 1
        }
    }

    impl ToString for $algorithm {
        fn to_string(&self) -> String {
            $string.to_string()
        }
    }

    impl ObjectHash for $algorithm {
        fn objecthash<H: ObjectHasher>(&self, hasher: &mut H) {
            self.to_string().objecthash(hasher);
        }
    }

    impl Serialize for $algorithm {
        fn serialize<O: OutputStream>(&self, _: &mut O) -> io::Result<()> {
            unimplemented!();
        }

        fn serialize_nested<O: OutputStream>(&self, field: u32, out: &mut O) -> io::Result<()> {
            out.write_varint(field, *self as u32 + 1)
        }
    }

    impl Deserialize for $algorithm {
        fn deserialize<R: io::Read>(_: &mut InputStream<R>) -> io::Result<$algorithm> {
            unimplemented!();
        }

        fn deserialize_nested<R: io::Read>(field: Field<R>) -> io::Result<$algorithm> {
            match try!(u32::deserialize_nested(field)) {
                1 => Ok($only),
                _ => Err(io::Error::new(
                         io::ErrorKind::InvalidInput,
                         concat!("unknown algorithm"))),
            }
        }
    }
));

impl_algorithm!(DigestAlgorithm, DigestAlgorithm::Sha256, "SHA256");
impl_algorithm!(EncryptionAlgorithm, EncryptionAlgorithm::Aes256Gcm, "AES256GCM");
impl_algorithm!(SignatureAlgorithm, SignatureAlgorithm::Ed25519, "Ed25519");
