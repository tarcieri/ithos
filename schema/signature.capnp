# A digital signature which optionally identifies the key that signed it

@0xe57df0ba88f961c5;

using SignatureAlg = import "alg.capnp".SignatureAlg;

struct Signature {
    # Signature algorithm used to author this signature
    algorithm @0 :SignatureAlg;

    # Raw signature serialized as bytes
    bytes @1 :Data;

    # Public key that authored this signature (raw)
    # TODO: switch to public key fingerprints
    publicKey @2 :Data;
}
