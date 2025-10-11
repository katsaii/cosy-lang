use std::process::ExitCode;
use std::path::{ Path, PathBuf };

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
    //let mut sess = Session::new();
    //parse_session(&mut sess, &args.file_path, args.lower);
    //err.submit(&sess);
}
/*
fn parse_session(sess : &mut Session, path : &Path, lower : bool) -> Option<()> {
    let file_data = match sess.manifest.load(path) {
        Ok(ok) => ok,
        Err(err) => {
            Diagnostic::error()
                .message(("failed to open file `{}`", [path.display().into()]))
                .note(("{}", [err.into()]))
                .report(&mut sess.issues);
            return None;
        },
    };
    let ast = Parser::parse(&mut sess.issues, &file_data);
    if lower {
        let hir = Ast2Hir::lower(&mut sess.issues, &ast);
        hir::debug_print_hir(&sess.manifest, &hir);
    } else {
        ast::debug_print_ast(&sess.manifest, &ast);
    }
    Some(())
}
*/