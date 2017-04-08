//! crypto/signing.rs: Digital signature functionality
//!
//! This module contains types for producing digital signatures. Digital signatures are primarily
//! used to authenticate and authorize changes to the directory tree.
//!
//! The Ed25519 digital signature algorithm (RFC 8032) is presently the only one supported
//!

use alg::{EncryptionAlg, SignatureAlg};
use block::{Block, Body};
use crypto;
use errors::*;
use object::credential::{self, Credential};
use objecthash;
use protobuf::RepeatedField;
use ring::rand::SecureRandom;
use ring::signature as signature_impl;
use rustc_serialize::base64::{self, ToBase64};
use signature::Signature;
use witness::Witness;

/// Digital signature keypair (includes public and private key)
pub struct KeyPair {
    /// The signature algorithm this key supports
    pub algorithm: SignatureAlg,
    keypair: signature_impl::Ed25519KeyPair,
}

impl<'a> KeyPair {
    #[cfg(test)]
    pub fn generate(rng: &SecureRandom) -> KeyPair {
        KeyPair {
            algorithm: SignatureAlg::Ed25519,
            keypair: signature_impl::Ed25519KeyPair::generate(rng).unwrap(),
        }
    }

    /// Generate a new `KeyPair` and seal it with the given encryption algorithm and key
    pub fn generate_and_seal(signature_alg: SignatureAlg,
                             encryption_alg: EncryptionAlg,
                             rng: &SecureRandom,
                             sealing_key: &[u8],
                             nonce: &[u8])
                             -> Result<(KeyPair, Vec<u8>)> {
        // Ed25519 is the only signature algorithm we presently support
        assert_eq!(signature_alg, SignatureAlg::Ed25519);

        let (keypair, serializable_keypair) =
            signature_impl::Ed25519KeyPair::generate_serializable(rng)?;


        let ciphertext = crypto::symmetric::seal(encryption_alg,
                                                 sealing_key,
                                                 nonce,
                                                 &serializable_keypair.private_key)?;

        let result = KeyPair {
            algorithm: SignatureAlg::Ed25519,
            keypair: keypair,
        };

        Ok((result, ciphertext))
    }

    /// Unseal an encrypted `KeyPair` from a `Credential` object
    pub fn unseal_from_credential(credential: &Credential,
                                  symmetric_key_bytes: &[u8])
                                  -> Result<KeyPair> {
        // Ed25519 is the only signature algorithm we presently support
        if credential.credential_type != credential::Type::SIGNATURE_KEY_PAIR {
            return Err(ErrorKind::KeyInvalid("not a signature key".to_string()).into());
        }

        KeyPair::unseal(SignatureAlg::Ed25519,
                        EncryptionAlg::AES256GCM,
                        symmetric_key_bytes,
                        &credential.encrypted_value,
                        &credential.public_key)
    }

    /// Unseal an encrypted `KeyPair`
    pub fn unseal(signature_alg: SignatureAlg,
                  encryption_alg: EncryptionAlg,
                  sealing_key: &[u8],
                  sealed_keypair: &[u8],
                  public_key: &[u8])
                  -> Result<KeyPair> {
        // Ed25519 is the only signature algorithm we presently support
        assert_eq!(signature_alg, SignatureAlg::Ed25519);

        let private_key =
            try!(crypto::symmetric::unseal(encryption_alg, sealing_key, sealed_keypair));

        let keypair = signature_impl::Ed25519KeyPair::from_bytes(&private_key, public_key)
            .chain_err(|| "not a valid Ed25519 keypair")?;

        Ok(KeyPair {
            algorithm: SignatureAlg::Ed25519,
            keypair: keypair,
        })
    }

    /// Return the public key for this `KeyPair`, serialized as bytes
    pub fn public_key_bytes(&'a self) -> &'a [u8] {
        self.keypair.public_key_bytes()
    }

    /// Sign the body of a block, returning a complete block with signature/witness data
    pub fn sign_block(&self, body: Body) -> Block {
        let mut message = String::from("ithos.block.body.ni:///sha-256;");
        message.push_str(&objecthash::digest(&body).as_ref().to_base64(base64::URL_SAFE));

        let signature = self.sign_raw_bytes(message.as_bytes());
        let mut witness = Witness::new();
        witness.set_signatures(RepeatedField::from_vec(vec![signature]));

        let mut block = Block::new();
        block.set_body(body);
        block.set_witness(witness);
        block
    }

    /// Compute a signature on a raw byte vector
    /// We avoid exposing this directly for domain separation reasons
    fn sign_raw_bytes(&self, msg: &[u8]) -> Signature {
        let mut proto = Signature::new();

        proto.set_algorithm(self.algorithm);
        proto.set_public_key(Vec::from(self.public_key_bytes()));
        proto.set_value(Vec::from(self.keypair.sign(msg).as_slice()));

        proto
    }
}

#[cfg(test)]
pub mod tests {
    use alg::{EncryptionAlg, SignatureAlg};
    use crypto::signing::KeyPair;
    use crypto::symmetric::{AES256GCM_KEY_SIZE, AES256GCM_NONCE_SIZE};
    use ring::rand;

    // WARNING: Please don't ever use zeroes as an actual encryption key
    const ENCRYPTION_KEY: [u8; AES256GCM_KEY_SIZE] = [0u8; AES256GCM_KEY_SIZE];

    #[test]
    fn test_sealing_and_unsealing() {
        let rng = rand::SystemRandom::new();

        let (keypair, sealed_keypair) = KeyPair::generate_and_seal(SignatureAlg::Ed25519,
                                                                   EncryptionAlg::AES256GCM,
                                                                   &rng,
                                                                   &ENCRYPTION_KEY,
                                                                   &[0u8; AES256GCM_NONCE_SIZE])
            .unwrap();

        let unsealed_keypair = KeyPair::unseal(SignatureAlg::Ed25519,
                                               EncryptionAlg::AES256GCM,
                                               &ENCRYPTION_KEY,
                                               &sealed_keypair,
                                               keypair.public_key_bytes())
            .unwrap();

        // *ring* verifies private key correctness when we call Ed25519KeyPair::from_bytes
        assert_eq!(keypair.public_key_bytes(),
                   unsealed_keypair.public_key_bytes());
    }
}
