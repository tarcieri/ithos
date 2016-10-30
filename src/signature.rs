use ring::{signature, aead};
use ring::rand::SecureRandom;

use algorithm::{EncryptionAlgorithm, SignatureAlgorithm};
use error::{Error, Result};

pub struct KeyPair(signature::Ed25519KeyPair);

impl<'a> KeyPair {
    #[cfg(test)]
    pub fn generate(rng: &SecureRandom) -> KeyPair {
        KeyPair(signature::Ed25519KeyPair::generate(rng).unwrap())
    }

    // Generate a new keypair and seal it with the given encryption algorithm and key
    // TODO: factor encryption code elsewhere (i.e. into an encryption-specific module)
    pub fn generate_and_seal(signature_alg: SignatureAlgorithm,
                             encryption_alg: EncryptionAlgorithm,
                             rng: &SecureRandom,
                             symmetric_key_bytes: &[u8],
                             nonce: &[u8])
                             -> Result<(KeyPair, Vec<u8>)> {
        // Ed25519 is the only signature algorithm we presently support
        assert!(signature_alg == SignatureAlgorithm::Ed25519);

        // Aes256Gcm is the only encryption algorithm we presently support
        assert!(encryption_alg == EncryptionAlgorithm::Aes256Gcm);

        let (keypair, serializable_keypair) =
            try!(signature::Ed25519KeyPair::generate_serializable(rng)
                .map_err(|_| Error::CryptoFailure));

        let symmetric_key = try!(aead::SealingKey::new(&aead::AES_256_GCM,
                                                       &symmetric_key_bytes[..])
            .map_err(|_| Error::CryptoFailure));

        let max_overhead_len = symmetric_key.algorithm().max_overhead_len();
        let mut buffer = Vec::with_capacity(serializable_keypair.private_key.len() +
                                            max_overhead_len);
        buffer.extend(&serializable_keypair.private_key);
        for _ in 0..max_overhead_len {
            buffer.push(0u8);
        }

        try!(aead::seal_in_place(&symmetric_key,
                                 &nonce,
                                 &mut buffer[..],
                                 max_overhead_len,
                                 &b""[..])
            .map_err(|_| Error::CryptoFailure));

        Ok((KeyPair(keypair), buffer))
    }

    pub fn unseal(symmetric_key_bytes: &[u8],
                  sealed_keypair: &[u8],
                  nonce: &[u8],
                  public_key: &[u8])
                  -> Result<KeyPair> {
        let symmetric_key = try!(aead::OpeningKey::new(&aead::AES_256_GCM,
                                                       &symmetric_key_bytes[..])
            .map_err(|_| Error::CryptoFailure));

        let mut buffer = Vec::from(sealed_keypair);

        let pt_len = try!(aead::open_in_place(&symmetric_key, nonce, 0, &mut buffer, &b""[..])
            .map_err(|_| Error::CryptoFailure));

        let keypair = try!(signature::Ed25519KeyPair::from_bytes(&buffer[..pt_len], &public_key)
            .map_err(|_| Error::CryptoFailure));

        Ok(KeyPair(keypair))
    }

    pub fn public_key_bytes(&'a self) -> &'a [u8] {
        self.0.public_key_bytes()
    }

    pub fn sign(&self, msg: &[u8]) -> signature::Signature {
        self.0.sign(msg)
    }
}

#[cfg(test)]
pub mod tests {
    // TODO: tests
}
