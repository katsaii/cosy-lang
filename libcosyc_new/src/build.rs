use std::{ env, io, io::IsTerminal };
use std::path::PathBuf;

use crate::src::SourceMap;
use crate::error::{ IssueManager, log, cli };
use crate::pretty::PrettyPrinter;

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

    /// Prints the errors reported by the current `IssueManager`, returning
    /// an exit code to return from the program.
    pub fn print_errors(
        &self,
        use_compact_errors : bool,
        use_colour : bool
    ) -> u8 {
        let (mut stderr, supports_colour) = stderr_from_env();
        let mut printer = PrettyPrinter::new(supports_colour && use_colour);
        let result = if use_compact_errors {
            log::write_errors(&mut printer, &mut stderr, &self.files, &self.issues)
        } else {
            cli::write_errors(&mut printer, &mut stderr, &self.files, &self.issues)
        };
        if let Err(err) = result {
            eprintln!("UNEXPECTED ERROR WHEN REPORTING ERRORS:\n{}", err);
            return 2;
        }
        if self.issues.has_errors() {
            return 1;
        }
        return 0;
    }
}

fn stderr_from_env() -> (io::Stderr, bool) {
    let stderr = io::stderr();
    let supports_colour = 'blk: {
        if !stderr.is_terminal() {
            break 'blk false;
        }
        if let Ok(val) = env::var("CLICOLOR_FORCE") {
            if val != "0" {
                break 'blk true;
            }
        }
        if env::var("NO_COLOR").is_ok() {
            break 'blk false;
        }
        if let Ok(val) = env::var("CLICOLOR_FORCE") {
            break 'blk val != "0";
        }
        true
    };
    (stderr, supports_colour)
}