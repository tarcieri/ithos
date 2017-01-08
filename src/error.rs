use std::error::Error as StdError;
use std::fmt;
use std::result::Result as StdResult;

#[derive(Debug, Eq, PartialEq)]
pub struct Error {
    pub kind: ErrorKind,
    pub msg: Option<String>,
}

impl Error {
    pub fn new(kind: ErrorKind, msg: Option<&str>) -> Error {
        Error {
            kind: kind,
            msg: msg.map(|s| s.to_string()),
        }
    }

    pub fn adapter(msg: Option<&str>) -> Error {
        Error::new(ErrorKind::Adapter, msg)
    }

    pub fn system(msg: Option<&str>) -> Error {
        Error::new(ErrorKind::System, msg)
    }

    pub fn rng(msg: Option<&str>) -> Error {
        Error::new(ErrorKind::Rng, msg)
    }

    pub fn parse(msg: Option<&str>) -> Error {
        Error::new(ErrorKind::Parse, msg)
    }

    pub fn serialize(msg: Option<&str>) -> Error {
        Error::new(ErrorKind::Serialize, msg)
    }

    pub fn transaction(msg: Option<&str>) -> Error {
        Error::new(ErrorKind::Transaction, msg)
    }

    pub fn ordering(msg: Option<&str>) -> Error {
        Error::new(ErrorKind::Ordering, msg)
    }

    pub fn path_invalid(msg: Option<&str>) -> Error {
        Error::new(ErrorKind::PathInvalid, msg)
    }

    pub fn not_found(msg: Option<&str>) -> Error {
        Error::new(ErrorKind::NotFound, msg)
    }

    pub fn entry_already_exists(msg: Option<&str>) -> Error {
        Error::new(ErrorKind::EntryAlreadyExists, msg)
    }

    pub fn nesting_invalid(msg: Option<&str>) -> Error {
        Error::new(ErrorKind::NestingInvalid, msg)
    }

    pub fn bad_type(msg: Option<&str>) -> Error {
        Error::new(ErrorKind::BadType, msg)
    }

    pub fn corrupt_data(msg: Option<&str>) -> Error {
        Error::new(ErrorKind::CorruptData, msg)
    }

    pub fn crypto_failure(msg: Option<&str>) -> Error {
        Error::new(ErrorKind::CryptoFailure, msg)
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum ErrorKind {
    Adapter,
    System,
    Rng,
    Parse,
    Serialize,
    Transaction,
    Ordering,
    PathInvalid,
    NotFound,
    EntryAlreadyExists,
    NestingInvalid,
    BadType,
    CorruptData,
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
            ErrorKind::CorruptData => "data was in a corrupt or unexpected state",
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

pub type Result<T> = StdResult<T, Error>;
