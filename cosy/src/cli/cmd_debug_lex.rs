use std::path::{ Path, PathBuf };
use libcosyc::{ Session, error::Diagnostic };
use libcosyc::parse::lex::{ Lexer, Token, self as lex };

/// Tokenises a file and outputs its lexical info.
///
/// Only operates on a single file.
#[derive(super::Args)]
pub(super) struct Args {
    /// Path of the `.cy` file to tokenise.
    #[arg()]
    file_path : PathBuf,
}

pub(super) fn execute(err : &mut super::ErrorReporter, args : Args) {
    let mut sess = Session::new();
    lex_session(&mut sess, &args.file_path);
    err.submit(&sess);
}

fn lex_session(sess : &mut Session, path : &Path) -> Option<()> {
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
    let file_meta = sess.manifest.get(file_data.id).unwrap();
    let src = &file_data.src;
    let mut lexer = Lexer::new(src);
    let mut tokens = vec![];
    loop {
        let token = lexer.next();
        let is_eof = token.1 == Token::EoF;
        tokens.push(token);
        if is_eof {
            break;
        }
    }
    lex::debug_print_tokens(path, src, &file_meta.lines, &tokens);
    Some(())
}