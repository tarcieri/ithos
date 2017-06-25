# ithos registry for cryptographic algorithms

@0xdff615305ae402d8;

# Hash functions
enum DigestAlg {
    # FIPS 180-2 Secure Hash Algorithm SHA-256 (RFC6234)
    sha256 @0;
}

# Symmetric encryption ciphers
enum EncryptionAlg {
    # Advanced Encryption Standard (AES) in Galois Counter Mode (GCM)
    aes256gcm @0;
}

# Digital signature algorithms
enum SignatureAlg {
    # Ed25519 Digital Signature Algorithm (RFC 8032)
    ed25519 @0;
}

# Password hashing functions
enum PasswordAlg {
    # The scrypt Password-Based Key Derivation Function (RFC 7914)
    scrypt  @0;
}

# Suite of SignatureAlg + EncryptionAlg + DigestAlg
enum CipherSuite {
    # Ed25519 + AES-256-GCM + SHA-256
    v0 @0;
}
