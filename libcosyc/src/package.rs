// TODO :: make this public

#[allow(unused_imports)]
use std::{
    env, fs, cmp, fmt, ops,
    ffi::{ OsStr, OsString },
    path::{ Path, PathBuf },
};

/// Stores information about a package.
#[derive(Debug)]
pub struct Package {
    /// A list of source files associated with package.
    pub files : Vec<File>,
    /// The name of this package.
    pub name : OsString,
}

impl Package {
    /// Creates a new package with the given name.
    pub fn new(name : OsString) -> Self {
        Self { files : Vec::new(), name }
    }

    /// Inserts a new virtual file into this package and returns its handle.
    pub fn load_str(
        &mut self,
        mut path : PathBuf,
        src : String,
    ) -> FileID {
        let name = path.file_stem().map(OsString::from).unwrap_or("main".into());
        let ext = path.extension().map(OsString::from);
        let dir = if path.pop() { Some(path) } else { None };
        let file_id = self.files.len();
        let mut file = File {
            dir, name, ext, src,
            lines : Vec::new(),
            file_id,
        };
        file.calculate_lines();
        self.files.push(file);
        file_id
    }

    /// Helper function for getting file information. Since `FileID`s are
    /// only ever created by the compiler, accessing an invalid file will
    /// panic.
    pub fn get_file_mut(&mut self, file : FileID) -> &mut File {
        &mut self.files[file]
    }

    /// Helper function for getting file information. Since `FileID`s are
    /// only ever created by the compiler, accessing an invalid file will
    /// panic.
    pub fn get_file(&self, file : FileID) -> &File {
        &self.files[file]
    }
}
