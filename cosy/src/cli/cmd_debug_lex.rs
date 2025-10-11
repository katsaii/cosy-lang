use std::process::ExitCode;
use std::path::{ Path, PathBuf };

use libcosyc::build::Session;
use libcosyc::error::Diagnostic;
use libcosyc::ir::ast::parse::lex;

/// Tokenises a file and outputs its lexical info.
///
/// Only operates on a single file.
#[derive(super::Args)]
pub(super) struct Args {
    /// Path of the `.cy` file to tokenise.
    #[arg()]
    file_path : PathBuf,
}

pub(super) fn execute(
    args_other : super::CommonArgs,
    args : Args,
) {
    let mut sess = Session::new();
    lex_session(args_other.printer, &mut sess, &args.file_path);
    sess.complete(args_other.printer, args_other.use_compact_errors);
}

fn lex_session(
    printer : super::PrinterTy,
    sess : &mut Session,
    path : &Path
) -> Option<()> {
    let file = match sess.files.load_file(path) {
        Ok(ok) => ok,
        Err(err) => {
            Diagnostic::from(err)
                .message(("failed to open file `{}`", [path.display().into()]))
                .report(&mut sess.issues);
            return None;
        },
    };
    lex::debug_write_tokens(printer, path, file.as_ref()).unwrap();
    Some(())
}