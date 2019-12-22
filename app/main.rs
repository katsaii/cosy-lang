use cosyc::{
    lexer::*,
    parser::*,
    evaluator::*
};

use std::fs;
use std::io::{
    Read,
    Write
};

fn main() {
    let inp = "tests/test.cosy";
    let mut inp = fs::OpenOptions::new()
            .read(true)
            .open(inp)
            .expect("unable to open file for reading");
    let out = "temp/log.txt";
    let _ = fs::remove_file(out);
    let mut out = fs::OpenOptions::new()
            .create(true)
            .write(true)
            .append(true)
            .open(out)
            .expect("unable to open file for writing");
    let mut source = String::new();
    inp.read_to_string(&mut source)
            .expect("unable to read from file");
    let scanner = StringScanner::from(&source);
    let lexer = Lexer::from(scanner);
    let result = Parser::parse(lexer);
    let s = match result {
        Ok(ast) => {
            let source = ast.to_string();
            format!("{}", source)
            /*let result = Interpreter::new().interpret(ast);
            match result {
                Ok(value) => format!("Source:\n{}\n\nValue:\n{:?}", source, "n/a"),
                Err(e) => format!("Runtime Error:\n{}", e)
            }*/
        },
        Err(es) => {
            es.iter().fold(String::from("Errors:"), |mut acc, e| {
                if !acc.is_empty() {
                    acc.push('\n');
                }
                acc.push_str(&e.to_string());
                acc
            })
        }
    };
    out.write(s.as_bytes())
            .expect("unable to write to file");
}