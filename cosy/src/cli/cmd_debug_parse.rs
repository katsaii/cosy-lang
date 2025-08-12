use libcosyc::parse::{ self as parse, ast, Parser };
use libcosyc::Session;

/// Parses the contents of a file and prints its untyped AST.
#[derive(super::Args)]
pub(super) struct Args {
    /// Path of the `.cy` file to parse.
    #[arg()]
    file_path : String,
}

pub(super) fn execute(sess : &mut Session, args : Args) {
    let module = parse::from_file(
        &mut sess.issues,
        &mut sess.files,
        &args.file_path
    );
    println!("{:#?}", module);
}