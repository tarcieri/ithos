use direntry::DirEntry;
use objectclass::ObjectClass;

#[derive(Debug, Eq, PartialEq)]
pub struct Entry<'a> {
    pub direntry: DirEntry<'a>,
    pub objectclass: ObjectClass,
}
