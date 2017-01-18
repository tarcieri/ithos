//! path.rs: Paths within the directory tree
//!
//! Functionality similar to std::path, but for paths within the ithos directory tree
//!

use objecthash::{ObjectHash, ObjectHasher};
use std::borrow::Borrow;
use std::hash::{Hash, Hasher};
use std::mem;

pub const SEPARATOR: &'static str = "/";

// Builder for absolute paths
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct PathBuf(String);

impl PathBuf {
    pub fn new() -> PathBuf {
        PathBuf(String::from(SEPARATOR))
    }

    pub fn as_path(&self) -> &Path {
        self.as_ref()
    }

    pub fn push<P: AsRef<str>>(&mut self, path: P) {
        if !self.0.ends_with(SEPARATOR) {
            self.0.push_str(SEPARATOR);
        }

        self.0.push_str(path.as_ref());
    }
}

impl From<String> for PathBuf {
    fn from(s: String) -> PathBuf {
        PathBuf(s)
    }
}

impl AsRef<str> for PathBuf {
    fn as_ref(&self) -> &str {
        &self.0[..]
    }
}

impl Into<String> for PathBuf {
    fn into(self) -> String {
        self.0
    }
}

impl Borrow<Path> for PathBuf {
    fn borrow(&self) -> &Path {
        Path::new(&self.0).unwrap()
    }
}

impl Hash for PathBuf {
    fn hash<H: Hasher>(&self, h: &mut H) {
        self.0.hash(h)
    }
}

impl ObjectHash for PathBuf {
    #[inline]
    fn objecthash<H: ObjectHasher>(&self, hasher: &mut H) {
        self.0.objecthash(hasher);
    }
}

// Slices of absolute paths
#[derive(Debug, Eq, PartialEq)]
pub struct Path(str);

impl Path {
    pub fn root() -> &'static Path {
        Path::new(SEPARATOR).unwrap()
    }

    pub fn new<S: AsRef<str> + ?Sized>(s: &S) -> Option<&Path> {
        if s.as_ref().starts_with(SEPARATOR) {
            Some(unsafe { mem::transmute(s.as_ref()) })
        } else {
            None
        }
    }

    // TODO: replace Vec with Components type ala std::path
    pub fn components(&self) -> Vec<&str> {
        if self.0 == *SEPARATOR {
            return Vec::new();
        }

        let mut result: Vec<&str> = self.0.split(SEPARATOR).collect();
        result.remove(0);
        result
    }

    pub fn parent(&self) -> Option<&Path> {
        let result = self.0.rsplitn(2, SEPARATOR).last();

        if result == Some("") {
            if self == Path::root() {
                return None;
            } else {
                return Some(Path::root());
            }
        }

        result.map(|component| Path::new(component).unwrap())
    }

    pub fn entry_name(&self) -> Option<&str> {
        self.0.rsplitn(2, SEPARATOR).next()
    }
}

impl AsRef<str> for Path {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl AsRef<Path> for Path {
    fn as_ref(&self) -> &Path {
        self
    }
}

impl AsRef<Path> for PathBuf {
    fn as_ref(&self) -> &Path {
        Path::new(&self.0).unwrap()
    }
}

impl ToString for Path {
    fn to_string(&self) -> String {
        String::from(&self.0)
    }
}

impl ToOwned for Path {
    type Owned = PathBuf;

    fn to_owned(&self) -> PathBuf {
        PathBuf(String::from(&self.0))
    }
}

impl Hash for Path {
    fn hash<H: Hasher>(&self, h: &mut H) {
        self.0.hash(h)
    }
}

impl ObjectHash for Path {
    #[inline]
    fn objecthash<H: ObjectHasher>(&self, hasher: &mut H) {
        self.0.objecthash(hasher);
    }
}

#[cfg(test)]
mod tests {
    use path::{Path, PathBuf};

    fn example_path() -> &'static Path {
        Path::new("/foo/bar/baz").unwrap()
    }

    #[test]
    fn pathbuf_inits_to_root() {
        assert_eq!(PathBuf::new().as_path(), Path::root());
    }

    #[test]
    fn pathbuf_push() {
        let mut pathbuf = PathBuf::new();
        pathbuf.push("foo");
        pathbuf.push("bar");
        pathbuf.push("baz");

        assert_eq!(pathbuf.as_path(), example_path());
    }

    #[test]
    fn path_parsing() {
        // Absolute paths are ok
        assert!(Path::new("/").is_some());
        assert!(Path::new("/foo").is_some());
        assert!(Path::new("/foo/bar").is_some());

        // Empty paths are not ok
        assert!(Path::new("").is_none());

        // Relative paths are not ok
        assert!(Path::new("../foo").is_none());
    }

    #[test]
    fn path_parent() {
        assert_eq!(Path::root().parent(), None);
        assert_eq!(Path::new("/foo").unwrap().parent().unwrap(), Path::root());
        assert_eq!(Path::new("/foo/bar").unwrap().parent().unwrap(),
                   Path::new("/foo").unwrap());
    }

    #[test]
    fn path_entry_name() {
        assert_eq!(example_path().entry_name().unwrap(), "baz");
    }

    #[test]
    fn root_components() {
        assert!(Path::root().components().is_empty());
    }

    #[test]
    fn path_components() {
        assert_eq!(example_path().components(), vec!["foo", "bar", "baz"])
    }
}
