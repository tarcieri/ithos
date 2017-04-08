//! error.rs: Error type family for ithos using error-chain

// Unfortunately macros and doc comments don't place nicely
#![allow(missing_docs)]

error_chain! {
    types {
        Error, ErrorKind, ResultExt, Result;
    }

    foreign_links {
        // Formatting error
        Fmt(::std::fmt::Error);

        // I/O error
        Io(::std::io::Error) #[cfg(unix)];

        // Cryptography error (as surfaced from *ring*)
        Crypto(::ring::error::Unspecified);

        // Protobuf error (as surfaced from rust-protobuf)
        Protobuf(::protobuf::ProtobufError);

        // LMDB error (as surfaced from lmdb-rs)
        Lmdb(::adapter::lmdb::LmdbError) #[cfg(feature = "lmdb-adapter")];
    }

    errors {
        ParseFailure(t: String) {
            description("unable to parse data")
            display("unable to parse data: '{}'", t)
        }

        SerializationFailure(t: String) {
            description("unable to serialize data")
            display("unable to serialize data: '{}'", t)
        }

        OrderingInvalid(t: String) {
            description("data is out-of-sequence with the expected order")
            display("data is out-of-sequence with the expected order: '{}'", t)
        }

        PathInvalid(t: String) {
            description("bad path syntax")
            display("bad path syntax: '{}'", t)
        }

        NotFound(t: String) {
            description("entry not found")
            display("entry not found: '{}'", t)
        }

        EntryAlreadyExists(t: String) {
            description("duplicate entry already exists")
            display("duplicate entry already exists: '{}'", t)
        }

        StructureInvalid(t: String) {
            description("entry type not allowed at this location")
            display("entry type not allowed at this location: '{}'", t)
        }

        TypeInvalid(t: String) {
            description("object of a different type expected")
            display("object of a different type expected: '{}'", t)
        }

        KeyInvalid(t: String) {
            description("cryptographic key of the wrong type")
            display("cryptographic key is invalid: {}", t)
        }

        CryptoFailure(t: String) {
            description("cryptographic operation failed")
            display("cryptographic operation failed: '{}'", t)
        }
    }
}
