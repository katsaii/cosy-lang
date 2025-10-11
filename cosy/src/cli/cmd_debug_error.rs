use std::process::ExitCode;
use std::path::{ Path, PathBuf };

use libcosyc::build::Session;
use libcosyc::error::{ Diagnostic, Severity };
use libcosyc::ir::ast::parse::lex::{ Lexer, Token };

/// Tokenises a file, reporting each token as an error. Used to test error
/// reporting.
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
    lex_session(&mut sess, &args.file_path);
    sess.complete(args_other.printer, args_other.use_compact_errors);
}

fn lex_session(
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
    let mut lexer = Lexer::new(&file.src);
    let span_start = lexer.peek_span().clone();
    loop {
        let token = lexer.next();
        let token_span = token.0;
        let token_name = format!("{:?}", token.1);
        let severity = match &token.1 {
            Token::Unknown(..) => Severity::Fatal,
            _ => Severity::Warning,
        };
        let mut diag = Diagnostic::new(severity)
            .message(("token name: {}", [token_name.into()]))
            .label((file.location(&token_span),
                ("span: {}", [format!("{:?}", token_span).into()]).into(),
            ));
        if lexer.peek_linebreak() {
            diag = diag.note("this token marks the end of a line");
        }
        diag.report(&mut sess.issues);
        if token.1 == Token::EoF {
            break;
        }
    }
    let span_end = lexer.peek_span();
    let span_full = span_start.join(span_end);
    Diagnostic::warning()
        .message("full span")
        .label((file.location(&span_full), 
                ("span: {}", [format!("{:?}", span_full).into()]).into(),
            ))
        .label_other((file.location(&span_start), "starts here".into()))
        .label_other((file.location(&span_end), "ends here".into()))
        .note("end-of-file tokens aren't rendered")
        .report(&mut sess.issues);
    Some(())
}