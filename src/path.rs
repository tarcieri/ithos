use std::hash::{Hash, Hasher};

use error::{Error, Result};

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Path {
    pub components: Vec<String>,
}

impl Path {
    pub fn new(string: &str) -> Result<Path> {
        let mut components: Vec<String> =
            string.split("/").map(|component| String::from(component)).collect();

        if components.is_empty() {
            return Err(Error::PathInvalid);
        }

        let prefix = components.remove(0);

        // Does the path start with something other than "/"?
        if !prefix.is_empty() {
            return Err(Error::PathInvalid);
        }

        Ok(Path { components: components })
    }

    pub fn to_string(&self) -> String {
        let mut result = String::new();

        for component in self.components.clone() {
            result.push_str("/");
            result.push_str(&component);
        }

        result
    }

    pub fn parent(&self) -> Path {
        if self.is_root() {
            return Path { components: vec![String::from("")] };
        }

        let mut parent_components = self.components.clone();
        parent_components.pop();

        Path { components: parent_components }
    }

    pub fn name(&self) -> String {
        self.components.last().unwrap().clone()
    }

    pub fn is_root(&self) -> bool {
        self.components.len() == 1 && self.components[0] == ""
    }
}

impl Hash for Path {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for component in &self.components {
            component.hash(state);
        }
    }
}
