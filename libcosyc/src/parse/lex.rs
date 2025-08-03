pub mod token;

pub use token::Token;
use crate::source::Span;
use std::{ mem, str::CharIndices };

/// Pairs a token with its byte span in the source code.
pub type TokenSpan = (Span, Token);

/// Tokenises Cosy source code into `Token`s on-demand. Does not report errors,
/// instead any errors are encoded into the tokens themselves.
pub struct Lexer<'a> {
    cursor : Cursor<'a>,
    peeked : TokenSpan,
}

impl<'a> Lexer<'a> {
    /// Creates a new lexer from the given source code.
    pub fn new(src : &'a str) -> Self {
        let mut cursor = Cursor::new(src);
        let peeked = cursor.read_token();
        Self { cursor, peeked }
    }

    /// Advances the lexer, returning the most recently peeked token.
    pub fn next(&mut self) -> TokenSpan {
        let peeked_new = self.cursor.read_token();
        mem::replace(&mut self.peeked, peeked_new)
    }

    /// Returns the type token currently peeked.
    pub fn peek(&self) -> &Token { &self.peeked.1 }
}

struct Cursor<'a> {
    src : &'a str,
    chars : CharIndices<'a>,
    peek_1 : (usize, char),
    peek_2 : (usize, char),
}

impl<'a> Cursor<'a> {
    fn new(src : &'a str) -> Cursor<'a> {
        let mut chars = src.char_indices();
        let peek_1 = Self::unwrap_char_idx(src, chars.next());
        let peek_2 = Self::unwrap_char_idx(src, chars.next());
        Self { src, chars, peek_1, peek_2 }
    }

    fn unwrap_char_idx(
        src : &str,
        char_index : Option<(usize, char)>
    ) -> (usize, char) {
        if let Some(x) = char_index { x } else { (src.len(), '\0') }
    }

    fn next(&mut self) -> (usize, char) {
        let peek_2 = Self::unwrap_char_idx(self.src, self.chars.next());
        let peek_1 = mem::replace(&mut self.peek_2, peek_2);
        mem::replace(&mut self.peek_1, peek_1)
    }

    fn next_while(&mut self, p : fn(char) -> bool) -> Option<(usize, char)> {
        let mut last = None;
        while p(self.peek_1.1) {
            last = Some(self.next());
        }
        return last;
    }

    fn read_token(&mut self) -> TokenSpan {
        let (offset_start, char_) = self.next();
        let token = match char_ {
            // symbols
            '(' => Token::LParen,
            ')' => Token::RParen,
            '[' => Token::LBox,
            ']' => Token::RBox,
            '{' => Token::LBrace,
            '}' => Token::RBrace,
            '=' => Token::Equal,
            ':' => {
                if self.peek_1.1 == ':' {
                    self.next();
                    Token::ColonColon
                } else {
                    Token::Colon
                }
            },
            '.' => {
                if self.peek_1.1 == '.' && self.peek_2.1 == '.' {
                    self.next();
                    self.next();
                    Token::LineContinue
                } else {
                    Token::Colon
                }
            },
            ',' => Token::Comma,
            ';' if self.peek_1.1 == ';' => {
                self.next();
                Token::LineBreak { implicit : false }
            },
            '-' if self.peek_1.1 == '-' => {
                self.next();
                self.next_while(|x| !(is_eol(x) || is_eof(x)));
                Token::Comment
            },
            // identifiers
            x if x == '_' || is_alpha(x) => {
                let is_hole = x == '_';
                self.next_while(|x| x == '_' || is_alpha(x) || is_digit(x));
                self.next_while(|x| x == '\''); // identifiers can end in '
                match &self.src[offset_start..self.peek_1.0] {
                    "begin" => Token::Begin,
                    "end" => Token::End,
                    "var" => Token::Var,
                    "fn" => Token::Fn,
                    "mod" => Token::Mod,
                    _ => Token::Id { is_hole }
                }
            },
            '`' => {
                self.next_while(|x| !(x == '`' || is_eol(x) || is_whitespace(x)));
                let unclosed = self.peek_1.1 != '`';
                Token::IdRaw { unclosed }
            },
            // numbers
            x if is_digit(x) => {
                let last_char = self.next_while(|x| x == '_' || is_digit(x));
                if matches!(last_char, Some((_, '_'))) {
                    Token::NumIntegral
                } else {
                    match self.peek_1.1 {
                        '.' if is_digit(self.peek_2.1) => {
                            self.next();
                            self.next_while(|x| x == '_' || is_digit(x));
                            Token::NumRational
                        },
                        'r' | 'R' if is_digit(self.peek_2.1) => {
                            self.next();
                            self.next_while(|x| x == '_' || is_digit_36(x));
                            Token::NumRadix
                        },
                        _ => Token::NumIntegral,
                    }
                }
            },
            // miscellaneous
            x if is_whitespace(x) => {
                self.next_while(is_whitespace);
                return self.read_token();
            },
            x if is_eol(x) => Token::LineBreak { implicit : true },
            x if is_eof(x) => Token::EoF,
            x => Token::Unknown(x),
        };
        let offset_end = self.peek_1.0;
        return (Span::new(offset_start..offset_end), token);
    }
}

fn is_eof(x : char) -> bool { x == '\0' }
fn is_eol(x : char) -> bool { matches!(x, '\n' | '\r') }
fn is_whitespace(x : char) -> bool { !is_eol(x) && x.is_whitespace() }
fn is_alpha(x : char) -> bool { x.is_ascii_alphabetic() }
fn is_digit(x : char) -> bool { x.is_ascii_digit() }
fn is_digit_36(x : char) -> bool { is_digit(x) || is_alpha(x) }