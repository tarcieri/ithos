use ring::digest;

pub static DIGEST_ALG: &'static digest::Algorithm = &digest::SHA256;

pub trait ObjectHash {
    fn objecthash(&self) -> digest::Digest;
}

// TODO: other integer types
impl ObjectHash for u64 {
    fn objecthash(&self) -> digest::Digest {
        let mut ctx = digest::Context::new(&DIGEST_ALG);

        // objecthash qualifier for integers
        ctx.update(b"i");
        ctx.update(self.to_string().as_bytes());

        ctx.finish()
    }
}

impl ObjectHash for str {
    fn objecthash(&self) -> digest::Digest {
        let mut ctx = digest::Context::new(&DIGEST_ALG);

        // objecthash qualifier for Unicode strings
        ctx.update(b"u");
        ctx.update(digest::digest(&DIGEST_ALG, &self.as_bytes()).as_ref());

        ctx.finish()
    }
}

// TODO: technically ObjectHash does not define a representation for binary data
// We should convert all binary data to e.g. Base64url first, then take the str
// objecthash of the encoded data
impl ObjectHash for [u8] {
    fn objecthash(&self) -> digest::Digest {
        let mut ctx = digest::Context::new(&DIGEST_ALG);

        // Pseudo-objecthash qualifier for octet strings
        ctx.update(b"o");
        ctx.update(digest::digest(&DIGEST_ALG, &self).as_ref());

        ctx.finish()
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
