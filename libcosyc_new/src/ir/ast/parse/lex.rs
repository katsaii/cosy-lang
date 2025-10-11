mod token;

use std::{ io, mem, str::CharIndices, path::Path };

use crate::src::{ Span, SourceFile };

pub use token::Token;

/// Pairs a token with its byte span in the source code.
pub type TokenSpan = (Span, Token);

/// Tokenises Cosy source code into `Token`s on-demand. Does not report errors,
/// instead any errors are encoded into the tokens themselves.
pub struct Lexer<'a> {
    cursor : Cursor<'a>,
    peeked : TokenSpan,
    peeked_linebreak : bool,
}

impl<'a> Lexer<'a> {
    /// Creates a new lexer from the given source code.
    pub fn new(src : &'a str) -> Lexer<'a> {
        let mut cursor = Cursor::new(src);
        let (peeked, peeked_linebreak) = Self::read_token_no_whitespace(&mut cursor);
        Self { cursor, peeked, peeked_linebreak }
    }

    /// Advances the lexer, returning the most recently peeked token.
    pub fn next(&mut self) -> TokenSpan {
        let (peeked, is_break) = Self::read_token_no_whitespace(&mut self.cursor);
        self.peeked_linebreak = is_break;
        mem::replace(&mut self.peeked, peeked)
    }

    fn read_token_no_whitespace(cursor : &mut Cursor<'a>) -> (TokenSpan, bool) {
        let mut is_break = false;
        let mut is_break_implicit = false;
        let mut is_continue = false;
        let peeked = loop {
            match Self::read_token_no_comment(cursor) {
                (_, Token::LineBreak { implicit }) => if implicit {
                    is_break_implicit = true;
                } else {
                    is_break = true;
                },
                (_, Token::LineContinue) => is_continue = true,
                x => break x,
            }
        };
        (peeked, is_break || is_break_implicit && !is_continue)
    }

    fn read_token_no_comment(cursor : &mut Cursor<'a>) -> TokenSpan {
        loop {
            match cursor.read_token() {
                (_, Token::Comment) => continue,
                x => break x,
            }
        }
    }

    /// Returns the type of the currently peeked token.
    pub fn peek(&self) -> &Token { &self.peeked.1 }

    /// Returns the span of the currently peeked token.
    pub fn peek_span(&self) -> &Span { &self.peeked.0 }

    /// Returns whether the there is a line break before the peeked token.
    ///
    /// This is Cosy's way of handling automatic semicolon insertion.
    pub fn peek_linebreak(&self) -> bool { self.peeked_linebreak }

    /// Returns a slice of the source code the lexer is parsing.
    pub fn slice(&self, span : &Span) -> &'a str { &span.slice(self.cursor.src) }
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
                self.next_while(|x| x == '_' || is_alpha(x) || is_digit(x));
                self.next_while(|x| x == '\''); // identifiers can end in '
                Token::from_keyword(&self.src[offset_start..self.peek_1.0])
            },
            '`' => {
                self.next_while(|x| !(x == '`' || is_eol(x)));
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

pub(super) fn is_eof(x : char) -> bool { x == '\0' }
pub(super) fn is_eol(x : char) -> bool { matches!(x, '\n' | '\r') }
pub(super) fn is_whitespace(x : char) -> bool { !is_eol(x) && x.is_whitespace() }
pub(super) fn is_alpha(x : char) -> bool { x.is_ascii_alphabetic() }
pub(super) fn is_digit(x : char) -> bool { x.is_ascii_digit() }
pub(super) fn is_digit_36(x : char) -> bool { is_digit(x) || is_alpha(x) }

const MAX_SRC_LENGTH : usize = 64;

use crate::pretty::{ PrettyPrinter, Colour, Decoration, Style };

/// Pretty prints a sequence of tokens for debugging purposes.
pub fn debug_write_tokens<W : io::Write>(
    printer : &mut PrettyPrinter<W>,
    file : &SourceFile,
) -> io::Result<()> {
    let src = &file.src;
    let mut lexer = Lexer::new(src);
    loop {
        let (span, token) = lexer.next();
        printer.write_style(Colour::BrightBlue)?;
        printer.write(&format!("{:?} ", span))?;
        printer.write_style(Decoration::Bold)?;
        printer.write(&format!("{:?} ", token))?;
        printer.write_style(Colour::Green)?;
        let token_str = span.slice(&src);
        if token_str.len() > MAX_SRC_LENGTH {
            printer.write("...")?;
        } else {
            printer.write(&format!("{:?}", token_str))?;
        }
        printer.clear_style()?;
        printer.write("\n")?;
        if token == Token::EoF {
            break;
        }
    }
    Ok(())
}