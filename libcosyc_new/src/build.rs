use std::path::PathBuf;

use crate::{ src, error };

/// Common info used throughout compilation.
pub struct Session {
    /// Stores all files managed by a compiler session.
    pub files : src::SourceMap,
    /// Stores any diagnostic information reported by the compiler tools.
    pub issues : error::IssueManager,
    /// The path to the build directory to write compiler files to if needed.
    pub build_dir : PathBuf,
}