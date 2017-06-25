# Encrypted Credentials (stored within ithos itself)

@0xa3790b8b931e6f32;

using Alg = import "/alg.capnp";
using Timestamp = UInt64;

# Credential type
enum Type {
    # Public/private keypair for producing digital signatures
    signatureKeyPair @0;
}

# Credential object
struct Credential {
    keyid @0 :Text;
    credentialType @1 :Type;
    credentialAlg @2 :Text;
    sealingAlg @3 :Alg.EncryptionAlg;
    encryptedValue @4 :Data;
    salt @5 :Data;
    publicKey @6 :Data;
    notBefore @7 :Timestamp;
    notAfter @8 :Timestamp;
    description @9 :Text;
}
