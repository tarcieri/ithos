//! crypto/symmetric.rs: Authenticated symmetric encryption with associated data (AEAD)
//!
//! AES-256-GCM is the only supported algorithm. Implementation provided by *ring*
//!

use alg::EncryptionAlg;
use error::{Error, Result};
use ring::aead;

pub const AES256GCM_KEY_SIZE: usize = 32;
pub const AES256GCM_NONCE_SIZE: usize = 12;

pub fn seal(algorithm: EncryptionAlg,
            secret_key: &[u8],
            nonce: &[u8],
            plaintext: &[u8])
            -> Result<Vec<u8>> {
    // AES256GCM is the only encryption algorithm we presently support
    assert!(algorithm == EncryptionAlg::AES256GCM);

    // Nonce must be the expected length
    if nonce.len() != AES256GCM_NONCE_SIZE {
        let msg = format!("nonce must be {} bytes (got {})",
                          AES256GCM_NONCE_SIZE,
                          nonce.len());
        return Err(Error::crypto_failure(Some(&msg)));
    }

    let sealing_key = try!(aead::SealingKey::new(&aead::AES_256_GCM, secret_key)
        .map_err(|_| Error::crypto_failure(None)));

    let tag_len = sealing_key.algorithm().tag_len();
    let mut buffer = Vec::with_capacity(AES256GCM_NONCE_SIZE + plaintext.len() + tag_len);

    buffer.extend_from_slice(nonce);
    buffer.extend_from_slice(plaintext);

    // Add space in the buffer to store the GCM tag
    for _ in 0..tag_len {
        buffer.push(0u8);
    }

    try!(aead::seal_in_place(&sealing_key,
                             &nonce,
                             &b""[..],
                             &mut buffer[AES256GCM_NONCE_SIZE..],
                             tag_len)
        .map_err(|_| Error::crypto_failure(None)));

    Ok(buffer)
}

pub fn unseal(algorithm: EncryptionAlg,
              secret_key: &[u8],
              ciphertext: &[u8])
              -> Result<Vec<u8>> {
    // AES256GCM is the only encryption algorithm we presently support
    assert!(algorithm == EncryptionAlg::AES256GCM);

    // The ciphertext must start with a valid nonce
    if ciphertext.len() < AES256GCM_NONCE_SIZE {
        return Err(Error::crypto_failure(Some("nonce missing from ciphertext")));
    }

    let opening_key = try!(aead::OpeningKey::new(&aead::AES_256_GCM, secret_key)
        .map_err(|_| Error::crypto_failure(Some("encryption key is corrupt"))));

    // Extract nonce from beginning of plaintext
    let nonce = &ciphertext[0..AES256GCM_NONCE_SIZE];
    let mut buffer = Vec::from(&ciphertext[AES256GCM_NONCE_SIZE..]);

    let pt_len = try!(aead::open_in_place(&opening_key, nonce, &b""[..], 0, &mut buffer)
            .map_err(|_| Error::crypto_failure(Some("decryption failed"))))
        .len();

    buffer.truncate(pt_len);
    Ok(buffer)
}

#[cfg(test)]
pub mod tests {
    use alg::EncryptionAlg;
    use crypto::symmetric::{self, AES256GCM_KEY_SIZE, AES256GCM_NONCE_SIZE};

    // WARNING: Please don't ever use zeroes as an actual encryption key
    const ENCRYPTION_KEY: [u8; AES256GCM_KEY_SIZE] = [0u8; AES256GCM_KEY_SIZE];
    const NONCE: [u8; AES256GCM_NONCE_SIZE] = [0u8; AES256GCM_NONCE_SIZE];
    const PLAINTEXT: &'static [u8] =
        b"BETWEEN SUBTLE SHADING AND THE ABSENCE OF LIGHT LIES THE NUANCE OF IQLUSION";

    #[test]
    fn test_sealing_and_unsealing() {
        let ciphertext = symmetric::seal(EncryptionAlg::AES256GCM,
                                         &ENCRYPTION_KEY,
                                         &NONCE,
                                         PLAINTEXT)
            .unwrap();

        let plaintext =
            symmetric::unseal(EncryptionAlg::AES256GCM, &ENCRYPTION_KEY, &ciphertext)
                .unwrap();

        assert_eq!(Vec::from(PLAINTEXT), plaintext);
    }
}
