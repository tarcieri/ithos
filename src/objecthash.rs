// Format-independent digests for structured data
// See: https://github.com/benlaurie/objecthash

use ring::digest;

pub static DIGEST_ALG: &'static digest::Algorithm = &digest::SHA256;

pub trait ObjectHash {
    fn objecthash(&self) -> digest::Digest;
}

#[inline]
pub fn digest(qualifier: &[u8], data: &[u8]) -> digest::Digest {
    let mut ctx = digest::Context::new(&DIGEST_ALG);

    ctx.update(qualifier);
    ctx.update(data);

    ctx.finish()
}

// TODO: other integer types
impl ObjectHash for u64 {
    fn objecthash(&self) -> digest::Digest {
        digest(b"i", self.to_string().as_bytes())
    }
}

impl ObjectHash for str {
    fn objecthash(&self) -> digest::Digest {
        digest(b"u", self.as_bytes())
    }
}

// TODO: technically ObjectHash does not define a representation for binary data
// We should convert all binary data to e.g. Base64url first, then take the str
// objecthash of the encoded data
impl ObjectHash for [u8] {
    fn objecthash(&self) -> digest::Digest {
        digest(b"o", self)
    }
}

impl<T: ObjectHash> ObjectHash for Vec<T> {
    fn objecthash(&self) -> digest::Digest {
        let mut ctx = digest::Context::new(&DIGEST_ALG);

        // objecthash qualifier for lists
        ctx.update(b"l");

        for item in self {
            ctx.update(item.objecthash().as_ref());
        }

        ctx.finish()
    }
}
