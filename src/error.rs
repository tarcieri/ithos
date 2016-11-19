use std::error::Error as StdError;
use std::fmt;
use std::result::Result as StdResult;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Error {
    Adapter(i32),
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
        match *self {
            Error::Adapter(_) => "storage adapter returned low-level error",
            Error::System => "a low-level system error occurred",
            Error::Rng => "the system random number generator returned an error",
            Error::Parse => "unable to parse data",
            Error::Serialize => "unable to serialize data",
            Error::Transaction => "the current transaction cannot be completed because of an error",
            Error::Ordering => "data is out-of-sequence with the expected order",
            Error::PathInvalid => "the given path is syntactically invalid",
            Error::NotFound => "the requested object was not found",
            Error::EntryAlreadyExists => "an attempt was made to insert a duplicate entry",
            Error::NestingInvalid => "attempted to add an object outside of allowed locations",
            Error::BadType => "expecting an object of a different type",
            Error::CorruptData => "data was in a corrupt or unexpected state",
            Error::CryptoFailure => "a cryptographic operation failed",
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{}", self.description())
    }
}

pub type Result<T> = StdResult<T, Error>;
