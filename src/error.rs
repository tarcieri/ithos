//! error.rs: Common Error and Result types
//!
//! A set of least common denominator error types which carry an optional dynamically generated
//! String description.
//!
//! These could probably be better specialized to individual cases
//!

use protobuf::ProtobufError;
use std::error::Error as StdError;
use std::fmt;
use std::result::Result as StdResult;

/// Custom error type used within ithos (TODO: switch to error-chain)
#[derive(Debug, Eq, PartialEq)]
pub struct Error {
    /// Type of error
    pub kind: ErrorKind,

    /// Optional string describing the error (TODO: replace with error-chain)
    pub msg: Option<String>,
}

impl Error {
    /// Create a new error of the given type
    pub fn new(kind: ErrorKind, msg: Option<&str>) -> Error {
        Error {
            kind: kind,
            msg: msg.map(|s| s.to_owned()),
        }
    }

    /// New ErrorKind::Adapter error
    pub fn adapter(msg: Option<&str>) -> Error {
        Error::new(ErrorKind::Adapter, msg)
    }

    /// New ErrorKind::System error
    pub fn system(msg: Option<&str>) -> Error {
        Error::new(ErrorKind::System, msg)
    }

    /// New ErrorKind::Rng error
    pub fn rng(msg: Option<&str>) -> Error {
        Error::new(ErrorKind::Rng, msg)
    }

    /// New ErrorKind::Parse error
    pub fn parse(msg: Option<&str>) -> Error {
        Error::new(ErrorKind::Parse, msg)
    }

    /// New ErrorKind::Serialize error
    pub fn serialize(msg: Option<&str>) -> Error {
        Error::new(ErrorKind::Serialize, msg)
    }

    /// New ErrorKind::Transaction error
    pub fn transaction(msg: Option<&str>) -> Error {
        Error::new(ErrorKind::Transaction, msg)
    }

    /// New ErrorKind::Ordering error
    pub fn ordering(msg: Option<&str>) -> Error {
        Error::new(ErrorKind::Ordering, msg)
    }

    /// New ErrorKind::PathInvalid error
    pub fn path_invalid(msg: Option<&str>) -> Error {
        Error::new(ErrorKind::PathInvalid, msg)
    }

    /// New ErrorKind::NotFound error
    pub fn not_found(msg: Option<&str>) -> Error {
        Error::new(ErrorKind::NotFound, msg)
    }

    /// New ErrorKind::EntryAlreadyExists error
    pub fn entry_already_exists(msg: Option<&str>) -> Error {
        Error::new(ErrorKind::EntryAlreadyExists, msg)
    }

    /// New ErrorKind::NestingInvalid error
    pub fn nesting_invalid(msg: Option<&str>) -> Error {
        Error::new(ErrorKind::NestingInvalid, msg)
    }

    /// New ErrorKind::BadType error
    pub fn bad_type(msg: Option<&str>) -> Error {
        Error::new(ErrorKind::BadType, msg)
    }

    /// New ErrorKind::CryptoFailure error
    pub fn crypto_failure(msg: Option<&str>) -> Error {
        Error::new(ErrorKind::CryptoFailure, msg)
    }
}

/// Type of error
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum ErrorKind {
    /// Storage adapter returned low-level error
    Adapter,

    /// A low-level system error occurred
    System,

    /// The system random number generator returned an error
    Rng,

    /// Unable to parse data
    Parse,

    /// Unable to serialize data
    Serialize,

    /// The transaction failed because of an error
    Transaction,

    /// Data is out-of-sequence with the expected order
    Ordering,

    /// The given path is syntactically invalid
    PathInvalid,

    /// The requested object was not found
    NotFound,

    /// An attempt was made to insert a duplicate entry
    EntryAlreadyExists,

    /// Attempted to add an object outside of allowed locations
    NestingInvalid,

    /// Expecting an object of a different type
    BadType,

    /// A cryptographic operation failed
    CryptoFailure,
}

impl StdError for Error {
    fn description(&self) -> &str {
        match self.kind {
            ErrorKind::Adapter => "storage adapter returned low-level error",
            ErrorKind::System => "a low-level system error occurred",
            ErrorKind::Rng => "the system random number generator returned an error",
            ErrorKind::Parse => "unable to parse data",
            ErrorKind::Serialize => "unable to serialize data",
            ErrorKind::Transaction => "the transaction failed because of an error",
            ErrorKind::Ordering => "data is out-of-sequence with the expected order",
            ErrorKind::PathInvalid => "the given path is syntactically invalid",
            ErrorKind::NotFound => "the requested object was not found",
            ErrorKind::EntryAlreadyExists => "an attempt was made to insert a duplicate entry",
            ErrorKind::NestingInvalid => "attempted to add an object outside of allowed locations",
            ErrorKind::BadType => "expecting an object of a different type",
            ErrorKind::CryptoFailure => "a cryptographic operation failed",
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        if let Some(ref msg) = self.msg {
            write!(fmt, "{}: {}", self.description(), msg)
        } else {
            write!(fmt, "{}", self.description())
        }
    }
}

/// Custom `Result` type which includes ithos's `Error` type
pub type Result<T> = StdResult<T, Error>;

impl From<ProtobufError> for Error {
    fn from(_error: ProtobufError) -> Error {
        Error::serialize(Some("protobuf error"))
    }
}
