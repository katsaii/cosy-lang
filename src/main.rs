mod runner;

use runner::lexer::Lexer;

fn main() {
    let mut lexer : Lexer = Lexer::new();
    lexer.add("LBRACE", r"\(");
    lexer.add("RBRACE", r"\)");
    lexer.add("IF", "if");
    lexer.add("MINUS", "-");
    lexer.add("ARROW", "->");
    match lexer.find_best_fit("if-> ", 2) {
        Some((name, l, r)) => println!("({}, {}, {})", name, l, r),
        None => println!("Unable to find a valid token.")
    }
}
