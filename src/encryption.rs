

use algorithm::EncryptionAlgorithm;
use error::{Error, Result};
use ring::aead;

pub const AES256GCM_KEY_SIZE: usize = 32;
pub const AES256GCM_NONCE_SIZE: usize = 12;

pub fn seal(algorithm: EncryptionAlgorithm,
            secret_key: &[u8],
            nonce: &[u8],
            plaintext: &[u8])
            -> Result<Vec<u8>> {
    // Aes256Gcm is the only encryption algorithm we presently support
    assert!(algorithm == EncryptionAlgorithm::Aes256Gcm);

    let sealing_key = try!(aead::SealingKey::new(&aead::AES_256_GCM, secret_key)
        .map_err(|_| Error::CryptoFailure));

    let max_overhead_len = sealing_key.algorithm().max_overhead_len();
    let mut buffer = Vec::with_capacity(AES256GCM_NONCE_SIZE + plaintext.len() + max_overhead_len);

    buffer.extend_from_slice(nonce);
    buffer.extend_from_slice(plaintext);

    // Add space in the buffer to store the GCM tag
    for _ in 0..max_overhead_len {
        buffer.push(0u8);
    }

    try!(aead::seal_in_place(&sealing_key,
                             &nonce,
                             &mut buffer[AES256GCM_NONCE_SIZE..],
                             max_overhead_len,
                             &b""[..])
        .map_err(|_| Error::CryptoFailure));

    Ok(buffer)
}

pub fn unseal(algorithm: EncryptionAlgorithm,
              secret_key: &[u8],
              ciphertext: &[u8])
              -> Result<Vec<u8>> {
    // Aes256Gcm is the only encryption algorithm we presently support
    assert!(algorithm == EncryptionAlgorithm::Aes256Gcm);

    // The sealed keypair MUST be larger than a nonce
    if ciphertext.len() <= AES256GCM_NONCE_SIZE {
        return Err(Error::CryptoFailure);
    }

    let opening_key = try!(aead::OpeningKey::new(&aead::AES_256_GCM, secret_key)
        .map_err(|_| Error::CryptoFailure));

    let nonce = &ciphertext[0..AES256GCM_NONCE_SIZE];
    let mut buffer = Vec::from(&ciphertext[AES256GCM_NONCE_SIZE..]);

    let pt_len = try!(aead::open_in_place(&opening_key, nonce, 0, &mut buffer, &b""[..])
        .map_err(|_| Error::CryptoFailure));

    buffer.truncate(pt_len);
    Ok(buffer)
}

#[cfg(test)]
pub mod tests {
    use algorithm::EncryptionAlgorithm;
    use encryption::{self, AES256GCM_KEY_SIZE, AES256GCM_NONCE_SIZE};

    // WARNING: Please don't ever use zeroes as an actual encryption key
    const ENCRYPTION_KEY: [u8; AES256GCM_KEY_SIZE] = [0u8; AES256GCM_KEY_SIZE];
    const NONCE: [u8; AES256GCM_NONCE_SIZE] = [0u8; AES256GCM_NONCE_SIZE];
    const PLAINTEXT: &'static [u8] =
        b"BETWEEN SUBTLE SHADING AND THE ABSENCE OF LIGHT LIES THE NUANCE OF IQLUSION";

    #[test]
    fn test_sealing_and_unsealing() {
        let ciphertext = encryption::seal(EncryptionAlgorithm::Aes256Gcm,
                                          &ENCRYPTION_KEY,
                                          &NONCE,
                                          PLAINTEXT)
            .unwrap();

        let plaintext =
            encryption::unseal(EncryptionAlgorithm::Aes256Gcm, &ENCRYPTION_KEY, &ciphertext)
                .unwrap();

        assert_eq!(Vec::from(PLAINTEXT), plaintext);
    }
}
