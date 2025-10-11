use std::path::{ Path, PathBuf };

use libcosyc::build::Session;
use libcosyc::error::Diagnostic;
use libcosyc::ir::ast;

/// Parses the contents of a file and prints its untyped AST.
#[derive(super::Args)]
pub(super) struct Args {
    /// Path of the `.cy` file to parse.
    #[arg()]
    file_path : PathBuf,
    /// Whether to lower the AST to HIR.
    #[arg(short, long)]
    lower : bool,
}

pub(super) fn execute(
    args_other : super::CommonArgs,
    args : Args,
) {
    let mut sess = Session::new();
    parse_session(args_other.printer, &mut sess, &args.file_path, args.lower);
    sess.complete(args_other.printer, args_other.use_compact_errors);
}

fn parse_session(
    printer : super::PrinterTy,
    sess : &mut Session,
    path : &Path,
    lower : bool,
) {
    let file = match sess.files.load_file(path) {
        Ok(ok) => ok,
        Err(err) => {
            Diagnostic::from(err)
                .message(("failed to open file `{}`", [path.display().into()]))
                .report(&mut sess.issues);
            return;
        },
    };
    let ast = ast::parse::from_file(&mut sess.issues, file.as_ref());
    if lower {
        //let hir = hir::lower::from_ast(&mut sess.issues, &ast);
        //hir::debug_write_hir(printer, &sess.files, &hir);
    } else {
        ast::debug_write_ast(printer, &sess.files, &ast).unwrap();
    }
}