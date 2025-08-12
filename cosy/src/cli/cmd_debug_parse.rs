use std::path::PathBuf;
use libcosyc::{ parse, Session };

/// Parses the contents of a file and prints its untyped AST.
#[derive(super::Args)]
pub(super) struct Args {
    /// Path of the `.cy` file to parse.
    #[arg()]
    file_path : PathBuf,
}

pub(super) fn execute(sess : &mut Session, args : Args) {
    if let Some(package) = parse::from_file(
        &mut sess.issues,
        &mut sess.files,
        args.file_path
    ) {
        println!("{:#?}", package);
    }
}