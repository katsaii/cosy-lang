use std::cmp;

use libcosyc::parse::lex::{ Lexer, Token };
use libcosyc::error::Diagnostic;

/// Tokenises a file and outputs its lexical info.
///
/// Only operates on a single file.
#[derive(super::Args)]
pub(super) struct Args {
    /// Path of the `.cy` file to tokenise.
    #[arg()]
    file_path : String,
}

const HEAD_SPAN : &str = "span";
const HEAD_NAME : &str = "name";
const HEAD_SRC : &str = "src";

const MAX_SRC_LENGTH : usize = 64;

pub(super) fn execute(sess : &mut crate::Session, args : Args) {
    let file_id = match sess.files.load((&args.file_path).into()) {
        Ok(x) => x,
        Err(err) => {
            Diagnostic::from(err).report(&mut sess.issues);
            return;
        },
    };
    let file = sess.files.get_file(file_id);
    let src = file.get_src();
    let mut lexer = Lexer::new(&src);
    let mut tokens = vec![];
    let mut max_span = HEAD_SPAN.len();
    let mut max_name = HEAD_NAME.len();
    loop {
        let token = lexer.next();
        let token_span = format!("{}", token.0);
        let token_name = format!("{:?}", token.1);
        let token_src = format!("{:?}", token.0.slice(&src));
        max_span = cmp::max(max_span, token_span.len());
        max_name = cmp::max(max_name, token_name.len());
        tokens.push((token_span, token_name, token_src));
        if token.1 == Token::EoF {
            break;
        }
    }
    println!("tokenisation output for: {}", args.file_path);
    println!(
        "{:<w_span$} | {:<w_name$} | {}",
        HEAD_SPAN, HEAD_NAME, HEAD_SRC, w_span=max_span, w_name=max_name
    );
    println!(
        "{:=<w_span$} | {:=<w_name$} | ===",
        "", "", w_span=max_span, w_name=max_name
    );
    for (span, name, src) in tokens {
        let src_trunc = if src.len() > MAX_SRC_LENGTH {
            format!("({} bytes)", src.len())
        } else { src };
        println!(
            "{:<w_span$} | {:<w_name$} | {}",
            span, name, src_trunc, w_span=max_span, w_name=max_name
        );
    }
}