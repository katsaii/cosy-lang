use std::path::PathBuf;
use libcosyc::{ parse, Session };

/// Parses the contents of a file and prints its untyped AST.
#[derive(super::Args)]
pub(super) struct Args {
    /// Path of the `.cy` file to parse.
    #[arg()]
    file_path : PathBuf,
}

pub(super) fn execute(_err : &mut super::ErrorReporter, _args : Args) {
    /*
    let mut sess = Session::default();
    if let Some(ast) = parse::package_from_file(
        &mut sess.issues,
        &mut sess.files,
        args.file_path,
    ) {
        println!("{:#?}", ast);
    }
    */
}