use std::fmt;
use std::result::Result as StdResult;
use std::error::Error as StdError;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Error {
    Rng,
    DbCreate,
    DbOpen,
    DbWrite,
    Parse,
    Serialize,
    Transaction,
    PathInvalid,
    NotFound,
    EntryAlreadyExists,
}

impl StdError for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Rng => "the system random number generator returned an error",
            Error::DbCreate => "unable to create database",
            Error::DbOpen => "unable to open database",
            Error::DbWrite => "unable to write to database",
            Error::Parse => "unable to parse data",
            Error::Serialize => "unable to serialize data",
            Error::Transaction => "the current transaction cannot be completed because of an error",
            Error::PathInvalid => "the given path is syntactically invalid",
            Error::NotFound => "the requested object was not found",
            Error::EntryAlreadyExists => "an attempt was made to insert a duplicate entry",
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{}", self.description())
    }
}

pub type Result<T> = StdResult<T, Error>;
