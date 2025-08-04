use crate::source::FileID;
use std::ffi::OsString;

/// Stores information about a package.
pub struct Package {
    /// A list of source files associated with package.
    pub files : Vec<FileID>,
    /// The name of this package.
    pub name : OsString,
}

impl Package {
    /// Creates a new package with the given name.
    pub fn new(name : OsString) -> Self {
        Self { files : Vec::new(), name }
    }
}
