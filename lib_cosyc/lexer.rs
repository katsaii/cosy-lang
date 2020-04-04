use crate::span::Span;

use std::str::CharIndices;
use std::mem;

/// A struct which converts a script into individual character spans.
pub struct Scanner<'a> {
    src : &'a str,
    chars : CharIndices<'a>,
    peek : CharKind,
    span : Span
}
impl<'a> Scanner<'a> {
    /// Creates a new parser session from this source code.
    pub fn from(src : &'a str) -> Self {
        let mut chars = src.char_indices();
        let first = chars
                .next()
                .map(|(_, c)| CharKind::identify(c))
                .unwrap_or(CharKind::EoF);
        Self {
            src,
            chars,
            peek : first,
            span : Span {
                line : 1,
                begin : 0,
                end : 0
            }
        }
    }

    /// Advances the scanner whilst some predicate `p` holds.
    pub fn advance_while(&mut self, p : fn(&CharKind) -> bool) {
        while p(&self.peek) {
            self.next();
        }
    }

    /// Returns the current peeked character.
    pub fn peek(&self) -> &CharKind {
        &self.peek
    }

    /// Advances the session scanner.
    pub fn next(&mut self) -> CharKind {
        if self.peek.is_valid_newline() {
            self.span.line += 1;
        }
        let next = if let Some((i, c)) = self.chars.next() {
            self.span.end = i;
            CharKind::identify(c)
        } else {
            self.span.end = self.src.len();
            CharKind::EoF
        };
        mem::replace(&mut self.peek, next)
    }

    /// Returns the current substring.
    pub fn substr(&self) -> &'a str {
        &self.src[self.span.begin..self.span.end]
    }

    /// Clears the current substring.
    pub fn clear_substr(&mut self) {
        self.span.begin = self.span.end;
    }
    
    /// Returns the current span.
    pub fn span(&self) -> Span {
        self.span.clone()
    }
}

/// An enum which stores character kinds.
#[derive(PartialEq, Debug, Clone)]
pub enum CharKind {
    Cr,
    Lf,
    Space,
    Digit,
    Graphic,
    Underscore,
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBox,
    RightBox,
    Dot,
    Comma,
    Colon,
    SemiColon,
    Dollar,
    Backtick,
    Hashtag,
    Address,
    DoubleQuote,
    SingleQuote,
    Bar,
    Caret,
    Ampersand,
    Bang,
    Hook,
    Equals,
    LessThan,
    GreaterThan,
    Plus,
    Minus,
    Tilde,
    Asterisk,
    ForwardSlash,
    BackSlash,
    Percent,
    EoF,
    Other
}
impl CharKind {
    /// Converts a character into its respective `CharKind`.
    pub fn identify(c : char) -> CharKind {
        use CharKind::*;
        match c {
            '\r' => Cr,
            '\n' => Lf,
            x if x.is_whitespace() => Space,
            x if x.is_ascii_digit() => Digit,
            x if x.is_alphanumeric() => Graphic,
            '_' => Underscore,
            '(' => LeftParen,
            ')' => RightParen,
            '{' => LeftBrace,
            '}' => RightBrace,
            '[' => LeftBox,
            ']' => RightBox,
            '.' => Dot,
            ',' => Comma,
            ':' => Colon,
            ';' => SemiColon,
            '$' => Dollar,
            '`' => Backtick,
            '#' => Hashtag,
            '@' => Address,
            '"' => DoubleQuote,
            '\'' => SingleQuote,
            | '|'
            | '¦' => Bar,
            '^' => Caret,
            '&' => Ampersand,
            '!' => Bang,
            '?' => Hook,
            '=' => Equals,
            '<' => LessThan,
            '>' => GreaterThan,
            '+' => Plus,
            '-' => Minus,
            '~' => Tilde,
            '*' => Asterisk,
            '/' => ForwardSlash,
            '\\' => BackSlash,
            '%' => Percent,
            _ => Other
        }
    }

    /// Returns whether the char is valid whitespace.
    pub fn is_valid_whitespace(&self) -> bool {
        if let CharKind::Space = self {
            true
        } else {
            self.is_valid_newline()
        }
    }

    /// Returns whether the char is a valid line ending.
    pub fn is_valid_ending(&self) -> bool {
        if let CharKind::EoF = self {
            true
        } else {
            self.is_valid_newline()
        }
    }

    /// Returns whether the char is valid new line character.
    pub fn is_valid_newline(&self) -> bool {
        if let CharKind::Lf = self {
            true
        } else {
            false
        }
    }

    /// Returns whether the char is a valid digit.
    pub fn is_valid_digit(&self) -> bool {
        if let CharKind::Digit = self {
            true
        } else {
            false
        }
    }

    /// Returns whether the char is a valid graphic.
    pub fn is_valid_graphic(&self) -> bool {
        use CharKind::*;
        if let
        | Graphic
        | Underscore
        | SingleQuote = self {
            true
        } else {
            false
        }
    }

    /// Returns whether the char is a valid operator.
    pub fn is_valid_operator(&self) -> bool {
        use CharKind::*;
        if let
        | Bar
        | Caret
        | Ampersand
        | Bang
        | Hook
        | Equals
        | LessThan
        | GreaterThan
        | Plus
        | Minus
        | Tilde
        | Asterisk
        | ForwardSlash
        | BackSlash
        | Percent
        | Other = self {
            true
        } else {
            false
        }
    }
}