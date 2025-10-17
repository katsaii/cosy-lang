use std::path::PathBuf;

use libcosyc::src::SourceMap;
use libcosyc::error::{ cli, Diagnostic, IssueManager };
use libcosyc::ir::ast;

/// Parses the contents of a file and prints its untyped AST.
#[derive(super::Args)]
pub(super) struct Args {
    /// Whether to lower the AST to HIR.
    #[arg(short, long)]
    lower : bool,
    /// Path of the `.cy` file to parse.
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
        let ast = ast::parse::from_file(&mut issues, file.as_ref());
        if args.lower {
            //let hir = hir::lower::from_ast(&mut issues, &ast);
            //hir::debug_write_hir(&mut cargs.printer, &files, &hir);
        } else {
            ast::debug_write_ast(&mut cargs.printer, &files, &ast).unwrap();
        }
    }
    cli::write_errors(&mut cargs.printer, &mut files, &mut issues).unwrap();
}