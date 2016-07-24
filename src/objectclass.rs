use std::string::ToString;
use ring::digest::Digest;

use objecthash::ObjectHash;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum ObjectClass {
    ROOT, // Root DSE
    SYSTEM, // System User (i.e. non-human/nonspecific-human)
}

impl ToString for ObjectClass {
    fn to_string(&self) -> String {
        match *self {
            ObjectClass::ROOT => "root".to_string(),
            ObjectClass::SYSTEM => "system".to_string(),
        }
    }
}

impl ObjectHash for ObjectClass {
    fn objecthash(&self) -> Digest {
        self.to_string().objecthash()
    }
}
