use std::{ fs, cmp };

use libcosyc::parse::lex;

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

pub(super) fn execute(args : Args) {
    let src = fs::read_to_string(&args.file_path).unwrap(); // TODO :: better errors
    let mut lexer = lex::Lexer::new(&src);
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
        if token.1 == lex::Token::EoF {
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
        println!(
            "{:<w_span$} | {:<w_name$} | {}",
            span, name, src, w_span=max_span, w_name=max_name
        );
    }
}