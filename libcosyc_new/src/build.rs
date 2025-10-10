use std::io;
use std::path::PathBuf;

use crate::src::SourceMap;
use crate::error::{ IssueManager, log, cli };
use crate::pretty;

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

    /// Writes any compiler errors using the supplied pretty printer.
    pub fn write_errors<W : io::Write>(
        &self,
        printer : &mut pretty::PrettyPrinter<W>,
        use_compact_errors : bool,
    ) -> io::Result<()> {
        if use_compact_errors {
            log::write_errors(printer, &self.files, &self.issues)
        } else {
            cli::write_errors(printer, &self.files, &self.issues)
        }
    }

    /// Writes any compiler errors to the standard error output.
    pub fn write_errors_to_stderr(
        &self,
        use_compact_errors : bool,
        use_colour : bool,
    ) -> io::Result<()> {
        let mut printer = pretty::from_env(use_colour);
        self.write_errors(&mut printer, use_compact_errors)
    }
}