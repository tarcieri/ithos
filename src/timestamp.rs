use objecthash::{ObjectHash, ObjectHasher};
use time;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct Timestamp(u64);

impl Timestamp {
    pub fn at(secs: u64) -> Timestamp {
        Timestamp(secs)
    }

    pub fn now() -> Timestamp {
        Timestamp(time::now_utc().to_timespec().sec as u64)
    }

    pub fn extend(&self, seconds: u64) -> Timestamp {
        Timestamp(self.0 + seconds)
    }

    pub fn to_int(&self) -> u64 {
        self.0
    }
}

impl ObjectHash for Timestamp {
    fn objecthash<H: ObjectHasher>(&self, hasher: &mut H) {
        self.0.objecthash(hasher);
    }
}
