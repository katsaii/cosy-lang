use std::path::PathBuf;
use libcosyc::Session;
use libcosyc::error::{ Diagnostic, Severity };
use libcosyc::parse::lex::{ Lexer, Token };
use libcosyc::vfs;

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

pub(super) fn execute(err : &mut super::ErrorReporter, args : Args) {
    let mut sess = Session::new();
    let file_data = match sess.manifest.load(&args.file_path) {
        Ok(ok) => ok,
        Err(err) => {
            Diagnostic::error()
                .message(("failed to open file `{}`", [args.file_path.display().into()]))
                .note(("{}", [err.into()]))
                .report(&mut sess.issues);
            return;
        }
    };
    let mut lexer = Lexer::new(&file_data.src);
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
            .label((file_data.location(&token_span), [
                ("span: {}", [format!("{:?}", token_span).into()]).into(),
            ]));
        if lexer.peek_linebreak() {
            diag = diag.label_other((file_data.location(&lexer.peek_span()), [
                ("next line continues here").into(),
            ]));
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
        .label((file_data.location(&span_full), [
                ("span: {}", [format!("{:?}", span_full).into()]).into(),
                "multiple captions are split over multiple lines nicely".into(),
            ]))
        .label_other((file_data.location(&span_start), [
                "starts here".into(),
            ]))
        .label_other((file_data.location(&span_end), [
                "ends here".into(),
            ]))
        .report(&mut sess.issues);
    err.submit(&sess)
}