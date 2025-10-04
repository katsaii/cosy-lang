use std::path::{ Path, PathBuf };
use libcosyc::{ Session, error::Diagnostic, parse::Parser };

/// Parses the contents of a file and prints its untyped AST.
#[derive(super::Args)]
pub(super) struct Args {
    /// Path of the `.cy` file to parse.
    #[arg()]
    file_path : PathBuf,
}

pub(super) fn execute(err : &mut super::ErrorReporter, args : Args) {
    let mut sess = Session::new();
    parse_session(&mut sess, &args.file_path);
    err.submit(&sess);
}

fn parse_session(sess : &mut Session, path : &Path) -> Option<()> {
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
    println!("{:#?}", ast);
    Some(())
}