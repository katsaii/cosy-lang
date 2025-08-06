use libcosyc::Session;
use libcosyc::error::{ Diagnostic, Severity };
use libcosyc::parse::lex::{ Lexer, Token };

/// Tokenises a file, reporting each token as an error. Used to test error
/// reporting.
///
/// Only operates on a single file.
#[derive(super::Args)]
pub(super) struct Args {
    /// Path of the `.cy` file to tokenise.
    #[arg()]
    file_path : String,
}

pub(super) fn execute(sess : &mut Session, args : Args) {
    let file_id = match sess.files.load((&args.file_path).into()) {
        Ok(x) => x,
        Err(diag) => {
            diag.report(&mut sess.issues);
            return;
        },
    };
    let file = sess.files.get_file(file_id);
    let src = file.get_src();
    let mut lexer = Lexer::new(&src);
    let span_start = lexer.peek_span().clone();
    loop {
        let token = lexer.next();
        let token_span = token.0;
        let token_name = format!("{:?}", token.1);
        let severity = match &token.1 {
            Token::Unknown(..) => Severity::Fatal,
            _ => Severity::Info,
        };
        Diagnostic::new(severity)
            .message(("token name: {}", [token_name.into()]))
            .label((file.make_location(&token_span), [
                ("span: {}", [format!("{}", token_span).into()]).into(),
            ]))
            .report(&mut sess.issues);
        if token.1 == Token::EoF {
            break;
        }
    }
    let span_end = lexer.peek_span();
    let span_full = span_start.join(span_end);
    Diagnostic::new(Severity::Info)
        .message("full span")
        .label((file.make_location(&span_full), [
                ("span: {}", [format!("{}", span_full).into()]).into(),
                "multiple captions are split over multiple lines nicely".into(),
            ]))
        .report(&mut sess.issues);
}