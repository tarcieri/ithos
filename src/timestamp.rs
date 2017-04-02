//! timestamp.rs: Unix time type
//!
//! All timestamps in the log are in the form of number of seconds since midnight GMT of
//! the Unix epoch (January 1st, 1970). Subsecond precision is not supported.
//!

use objecthash::{ObjectHash, ObjectHasher};
use time;

/// Number of seconds since the Unix epoch
// TODO: better specification of exact semantics and relationship to UTC, TAI, etc.
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct Timestamp(u64);

impl Timestamp {
    /// Obtain the `Timestamp` at a given integer number of seconds.
    pub fn at(secs: u64) -> Timestamp {
        Timestamp(secs)
    }

    /// Obtain a `Timestamp` for the current time
    pub fn now() -> Timestamp {
        Timestamp(time::now_utc().to_timespec().sec as u64)
    }

    /// Obtain the `Timestamp` that's the given number of seconds from this one
    pub fn extend(&self, seconds: u64) -> Timestamp {
        Timestamp(self.0 + seconds)
    }

    /// Convert `Timestamp` to an integer representing Unix time
    pub fn to_int(&self) -> u64 {
        self.0
    }
}

impl ObjectHash for Timestamp {
    fn objecthash<H: ObjectHasher>(&self, hasher: &mut H) {
        self.0.objecthash(hasher);
    }
}
