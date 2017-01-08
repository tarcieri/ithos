use algorithm::{EncryptionAlgorithm, SignatureAlgorithm};
use buffoon::{OutputStream, Serialize};
use encryption;
use error::{Error, Result};
use objecthash::{self, ObjectHash, ObjectHasher};
use proto::ToProto;
use ring::rand::SecureRandom;
use ring::signature as signature_impl;
use std::io;

#[derive(Debug, Eq, PartialEq)]
pub struct Signature {
    pub algorithm: SignatureAlgorithm,
    pub public_key: Vec<u8>,
    pub value: Vec<u8>,
}

impl ToProto for Signature {}

impl Serialize for Signature {
    fn serialize<O: OutputStream>(&self, out: &mut O) -> io::Result<()> {
        try!(out.write(1, &self.algorithm));
        try!(out.write(2, &self.public_key));
        try!(out.write(3, &self.value));

        Ok(())
    }
}

impl ObjectHash for Signature {
    #[inline]
    fn objecthash<H: ObjectHasher>(&self, hasher: &mut H) {
        objecthash_struct!(
            hasher,
            "algorithm" => self.algorithm.to_string(),
            "public_key" => self.public_key,
            "value" => self.value
        )
    }
}

pub struct KeyPair {
    pub algorithm: SignatureAlgorithm,
    keypair: signature_impl::Ed25519KeyPair,
}

impl<'a> KeyPair {
    #[cfg(test)]
    pub fn generate(rng: &SecureRandom) -> KeyPair {
        KeyPair {
            algorithm: SignatureAlgorithm::Ed25519,
            keypair: signature_impl::Ed25519KeyPair::generate(rng).unwrap(),
        }
    }

    // Generate a new keypair and seal it with the given encryption algorithm and key
    pub fn generate_and_seal(signature_alg: SignatureAlgorithm,
                             encryption_alg: EncryptionAlgorithm,
                             rng: &SecureRandom,
                             sealing_key: &[u8],
                             nonce: &[u8])
                             -> Result<(KeyPair, Vec<u8>)> {
        // Ed25519 is the only signature algorithm we presently support
        assert!(signature_alg == SignatureAlgorithm::Ed25519);

        let (keypair, serializable_keypair) =
            try!(signature_impl::Ed25519KeyPair::generate_serializable(rng)
                .map_err(|_| Error::crypto_failure(None)));

        let ciphertext = try!(encryption::seal(encryption_alg,
                                               sealing_key,
                                               nonce,
                                               &serializable_keypair.private_key));

        let result = KeyPair {
            algorithm: SignatureAlgorithm::Ed25519,
            keypair: keypair,
        };

        Ok((result, ciphertext))
    }

    pub fn unseal(signature_alg: SignatureAlgorithm,
                  encryption_alg: EncryptionAlgorithm,
                  sealing_key: &[u8],
                  sealed_keypair: &[u8],
                  public_key: &[u8])
                  -> Result<KeyPair> {
        // Ed25519 is the only signature algorithm we presently support
        assert!(signature_alg == SignatureAlgorithm::Ed25519);

        let private_key = try!(encryption::unseal(encryption_alg, sealing_key, sealed_keypair));

        let keypair = try!(signature_impl::Ed25519KeyPair::from_bytes(&private_key, &public_key)
            .map_err(|_| Error::crypto_failure(None)));

        Ok(KeyPair {
            algorithm: SignatureAlgorithm::Ed25519,
            keypair: keypair,
        })
    }

    pub fn public_key_bytes(&'a self) -> &'a [u8] {
        self.keypair.public_key_bytes()
    }

    pub fn sign(&self, msg: &[u8]) -> Signature {
        let signature = self.keypair.sign(msg);

        Signature {
            algorithm: self.algorithm,
            public_key: Vec::from(self.public_key_bytes()),
            value: Vec::from(signature.as_slice()),
        }
    }
}

#[cfg(test)]
pub mod tests {
    use algorithm::{EncryptionAlgorithm, SignatureAlgorithm};
    use encryption::{AES256GCM_KEY_SIZE, AES256GCM_NONCE_SIZE};
    use ring::rand;
    use signature::KeyPair;

    // WARNING: Please don't ever use zeroes as an actual encryption key
    const ENCRYPTION_KEY: [u8; AES256GCM_KEY_SIZE] = [0u8; AES256GCM_KEY_SIZE];

    #[test]
    fn test_sealing_and_unsealing() {
        let rng = rand::SystemRandom::new();

        let (keypair, sealed_keypair) = KeyPair::generate_and_seal(SignatureAlgorithm::Ed25519,
                                                                   EncryptionAlgorithm::Aes256Gcm,
                                                                   &rng,
                                                                   &ENCRYPTION_KEY,
                                                                   &[0u8; AES256GCM_NONCE_SIZE])
            .unwrap();

        let unsealed_keypair = KeyPair::unseal(SignatureAlgorithm::Ed25519,
                                               EncryptionAlgorithm::Aes256Gcm,
                                               &ENCRYPTION_KEY,
                                               &sealed_keypair,
                                               keypair.public_key_bytes())
            .unwrap();

        // *ring* verifies private key correctness when we call Ed25519KeyPair::from_bytes
        assert_eq!(keypair.public_key_bytes(),
                   unsealed_keypair.public_key_bytes());
    }
}
