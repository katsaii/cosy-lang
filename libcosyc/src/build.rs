use std::path::PathBuf;

use crate::src::SourceMap;
use crate::error::IssueManager;

/// Common info used throughout compilation.
pub struct Session {
    /// Stores all files managed by a compiler session.
    pub files : SourceMap,
    /// Stores any diagnostic information reported by the compiler tools.
    pub issues : IssueManager,
    /// The path to the build directory to write compiler files to if needed.
    pub build_dir : PathBuf,
}

impl Session {
    pub fn new() -> Self {
        Self {
            files : SourceMap::default(),
            issues : IssueManager::default(),
            build_dir : PathBuf::from("build"),
        }
    }

    /// Parses a module into its HIR representation, inferring the types of
    /// all variables as best it can at this stage.
    pub fn build_module(&mut self, ) -> () {
        //let file = match sess.files.load_file(path) {
        //    Ok(ok) => ok,
        //    Err(err) => {
        //        Diagnostic::from(err)
        //            .message(("failed to open file `{}`", [path.display().into()]))
        //            .report(&mut sess.issues);
        //        return;
        //    },
        //};
    }
}