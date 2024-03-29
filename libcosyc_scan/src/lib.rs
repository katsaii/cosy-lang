pub mod symbol;
pub mod token;
pub mod reader;

use reader::SymbolReader;
use symbol::SymbolKind;
use token::TokenKind;
use libcosyc_diagnostic::source::Span;

/// Converts a string slice into lexemes, ignoring whitespace.
pub struct Lexer<'a> {
    reader : SymbolReader<'a>,
    ignore_next_symbol : bool
}

impl<'a> Lexer<'a> {
    /// Returns the span of the current lexeme.
    pub fn span(&self) -> &Span {
        self.reader.span()
    }

    /// Returns the substring of the current lexeme.
    pub fn substring(&self) -> &'a str {
        self.reader.substring()
    }

    /// Returns the next token of the source.
    pub fn generate_token(&mut self) -> TokenKind {
        if self.ignore_next_symbol {
            self.reader.advance();
            self.ignore_next_symbol = false;
        }
        self.reader.reset_span();
        match self.reader.advance() {
            x if x.is_valid_whitespace() => {
                self.reader.advance_while(SymbolKind::is_valid_whitespace);
                TokenKind::Whitestuff
            },
            SymbolKind::LeftParen => TokenKind::LeftParen,
            SymbolKind::RightParen => TokenKind::RightParen,
            SymbolKind::LeftBox => TokenKind::LeftBox,
            SymbolKind::RightBox => TokenKind::RightBox,
            SymbolKind::LeftBrace => TokenKind::LeftBrace,
            SymbolKind::RightBrace => TokenKind::RightBrace,
            SymbolKind::Colon => TokenKind::Colon,
            x if x.is_valid_digit() => {
                self.reader.advance_while(SymbolKind::is_valid_digit);
                TokenKind::Integral
            },
            x if x.is_valid_graphic() => {
                self.reader.advance_while(SymbolKind::is_valid_graphic);
                // alphabetic identifiers can end with any number of `'` (called "prime")
                self.reader.advance_while(|x| matches!(x, SymbolKind::SingleQuote));
                match self.substring() {
                    "_" => TokenKind::Hole,
                    "let" => TokenKind::Let,
                    _ => TokenKind::Identifier
                }
            },
            x if x.is_valid_operator() => {
                self.reader.advance_while(SymbolKind::is_valid_operator);
                if self.reader.holds_comment_lexeme() {
                    self.reader.reset_span();
                    self.reader.advance_while(|x| !matches!(x, SymbolKind::EoL));
                    TokenKind::Comment
                } else {
                    TokenKind::Operator { precedence : 0 }
                }
            },
            SymbolKind::Backtick => {
                self.reader.reset_span(); // this is used so that identifiers Foo and `Foo` are the same
                self.reader.advance_while(|x| !matches!(x, SymbolKind::Backtick | SymbolKind::EoL));
                let closed = matches!(self.reader.peek(), SymbolKind::Backtick);
                self.ignore_next_symbol = closed;
                TokenKind::RawIdentifier { closed }
            }
            SymbolKind::EoF => TokenKind::EoF,
            _ => TokenKind::Unknown
        }
    }
}

impl<'a> From<SymbolReader<'a>> for Lexer<'a> {
    fn from(reader : SymbolReader<'a>) -> Self {
        let ignore_next_symbol = false;
        Self { reader, ignore_next_symbol }
    }
}

impl<'a> From<&'a str> for Lexer<'a> {
    fn from(src : &'a str) -> Self {
        Lexer::from(SymbolReader::from(src))
    }
}

impl Into<Span> for Lexer<'_> {
    fn into(mut self) -> Span {
        self.reader.advance_while(|x| !matches!(x, SymbolKind::EoF));
        self.span().clone()
    }
}
