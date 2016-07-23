use ring::{rand, signature, aead};
use ring::rand::SecureRandom;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum SignatureAlgorithm {
    Ed25519,
}

pub struct KeyPair(signature::Ed25519KeyPair);

impl<'a> KeyPair {
    #[cfg(test)]
    pub fn generate(rng: &SecureRandom) -> KeyPair {
        KeyPair(signature::Ed25519KeyPair::generate(rng).unwrap())
    }

    pub fn generate_and_seal(alg: SignatureAlgorithm,
                             rng: &SecureRandom,
                             symmetric_key_bytes: &[u8])
                             -> (KeyPair, Vec<u8>) {
        // Ed25519 is the only password hashing algorithm we support for now
        assert!(alg == SignatureAlgorithm::Ed25519);

        let (keypair, serializable_keypair) = signature::Ed25519KeyPair::generate_serializable(rng)
            .unwrap();

        let symmetric_key = aead::SealingKey::new(&aead::AES_256_GCM, &symmetric_key_bytes[..])
            .unwrap();

        let max_overhead_len = symmetric_key.algorithm().max_overhead_len();
        let mut buffer = Vec::with_capacity(serializable_keypair.private_key.len() +
                                            max_overhead_len);
        buffer.extend(&serializable_keypair.private_key);
        for _ in 0..max_overhead_len {
            buffer.push(0u8);
        }

        // TODO: store a counter for the nonce and increment each time a password is changed
        // This should ensure we never reuse a nonce, even if the user reuses a password
        let nonce = [0u8; 12];
        aead::seal_in_place(&symmetric_key,
                            &nonce,
                            &mut buffer[..],
                            max_overhead_len,
                            &b""[..])
            .unwrap();

        // TODO: protos for storing keypairs
        (KeyPair(keypair), buffer)
    }

    pub fn algorithm(&self) -> SignatureAlgorithm {
        SignatureAlgorithm::Ed25519
    }

    pub fn public_key_bytes(&'a self) -> &'a [u8] {
        self.0.public_key_bytes()
    }

    pub fn sign(&self, msg: &[u8]) -> signature::Signature {
        self.0.sign(msg)
    }
}
