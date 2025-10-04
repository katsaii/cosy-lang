pub mod source;
pub mod vfs;
pub mod error;
pub mod parse;
pub mod lower;
pub mod r#gen;
pub mod reporting;

use std::path::PathBuf;

/// Cosy language file extension.
pub const EXT_SRC : &'static str = "cy";

/// Cosy IR file extension.
pub const EXT_IR : &'static str = "casm";

/// Common info used throughout compilation of a package.
pub struct Session {
    /// Stores all files managed by a compiler session.
    pub manifest : vfs::Manifest,
    /// Stores any diagnostic information reported by the compiler tools.
    pub issues : error::IssueManager,
    /// The path to the build directory to write compiler files to if needed.
    pub build_dir : PathBuf,
}

impl Session {
    pub fn new() -> Self {
        Self {
            manifest : vfs::Manifest::default(),
            issues : error::IssueManager::default(),
            build_dir : PathBuf::from("build"),
        }
    }
}