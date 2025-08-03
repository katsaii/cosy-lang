use libcosyc::parse::lex;

pub(super) fn main() {
    let src = "-- simple main function
fn main() = begin
    var m = 3
    var n = m
    --var n = if condition then 1 else 2 end
    --        ... + m * 3
    n
end";
    let mut lexer = lex::Lexer::new(src);
    loop {
        let token = lexer.next();
        println!("{:?}\t = \t{:?}", token.0.slice(src), token.1);
        if token.1 == lex::Token::EoF {
            break;
        }
    }
}