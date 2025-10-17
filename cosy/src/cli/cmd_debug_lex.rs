use std::path::PathBuf;

use libcosyc::src::SourceMap;
use libcosyc::error::{ cli, Diagnostic, IssueManager };
use libcosyc::ir::ast::parse::lex;

/// Tokenises a file and outputs its lexical info.
///
/// Only operates on a single file.
#[derive(super::Args)]
pub(super) struct Args {
    /// Path of the `.cy` file to tokenise.
    #[arg()]
    path : PathBuf,
}

pub(super) fn execute(mut cargs : super::CommonArgs, args : Args) {
    let mut issues = IssueManager::default();
    let mut files = SourceMap::default();
    'task: {
        let file = match files.load_file(&args.path) {
            Ok(ok) => ok,
            Err(err) => {
                Diagnostic::from(err)
                    .message(("failed to open file `{}`", [
                        args.path.display().into(),
                    ]))
                    .report(&mut issues);
                break 'task;
            },
        };
        lex::debug_write_tokens(&mut cargs.printer, &args.path, file.as_ref()).unwrap();
    }
    cli::write_errors(&mut cargs.printer, &mut files, &mut issues).unwrap();
}