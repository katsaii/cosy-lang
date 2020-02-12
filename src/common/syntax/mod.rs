use super::Context;

/// Stores a token and its location in the source file.
#[derive(Debug, Clone)]
pub struct Token {
    pub kind : TokenKind,
    pub context : Context
}

/// An enum which describes available token types.
#[derive(PartialEq, Debug, Clone)]
pub enum TokenKind {
    Keyword(KeywordKind),
    Symbol(SymbolKind),
    Identifier(IdentifierKind),
    Literal(LiteralKind),
    EoF,
    Directive,
    Unknown
}
impl TokenKind {
    /// Returns `true` of the token is a keyword.
    pub fn is_keyword(&self) -> bool {
        if let TokenKind::Keyword(..) = self { true } else { false }
    }

    /// Returns `true` of the token is an identifier.
    pub fn is_identifier(&self) -> bool {
        if let TokenKind::Identifier(..) = self { true } else { false }
    }

    /// Returns `true` of the token is an operator.
    pub fn is_operator(&self) -> bool {
        if let TokenKind::Identifier(
                IdentifierKind::AlphaNumeric) = self { false } else { true }
    }

    /// Returns `true` of the token is a literal.
    pub fn is_literal(&self) -> bool {
        if let TokenKind::Literal(..) = self { true } else { false }
    }

    /// Returns `true` of the token is the end of the file.
    pub fn is_eof(&self) -> bool {
        if let TokenKind::EoF = self { true } else { false }
    }

    /// Returns `true` of the token is a compiler directive.
    pub fn is_directive(&self) -> bool {
        if let TokenKind::Directive = self { true } else { false }
    }

    /// Returns `true` of the token is unknown.
    pub fn is_unknown(&self) -> bool {
        if let TokenKind::Unknown = self { true } else { false }
    }
}

/// An enum which describes available keyword types.
#[derive(PartialEq, Debug, Clone)]
pub enum KeywordKind {
    Var,
    If,
    Else,
    Then
}

/// An enum which describes available symbol types.
#[derive(PartialEq, Debug, Clone)]
pub enum SymbolKind {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    SemiColon,
    Dollar,
    Backtick,
    Address
}

/// An enum which describes available identifier types.
#[derive(PartialEq, Debug, Clone)]
pub enum IdentifierKind {
    AlphaNumeric,
    Bar,
    Caret,
    Ampersand,
    Bang,
    Equals,
    LessThan,
    GreaterThan,
    Plus,
    Minus,
    Asterisk,
    ForwardSlash,
    BackSlash,
    Percent,
    Other
}

/// An enum which describes available literal types.
#[derive(PartialEq, Debug, Clone)]
pub enum LiteralKind {
    Character,
    Integer,
    Real
}

/// An enum which stores character kinds.
#[derive(PartialEq, Debug, Clone)]
pub enum CharKind {
    Whitespace,
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
    Operator,
    NewLine,
    EoF
}
impl CharKind {
    /// Converts a character into its respective `CharKind`.
    pub fn identify(c : char) -> CharKind {
        match c {
            '\n' => CharKind::NewLine,
            x if x.is_whitespace() => CharKind::Whitespace,
            x if x.is_ascii_digit() => CharKind::Digit,
            x if x.is_alphanumeric() => CharKind::Graphic,
            '_' => CharKind::Underscore,
            '(' => CharKind::LeftParen,
            ')' => CharKind::RightParen,
            '{' => CharKind::LeftBrace,
            '}' => CharKind::RightBrace,
            '[' => CharKind::LeftBox,
            ']' => CharKind::RightBox,
            '.' => CharKind::Dot,
            ',' => CharKind::Comma,
            ':' => CharKind::Colon,
            ';' => CharKind::SemiColon,
            '$' => CharKind::Dollar,
            '`' => CharKind::Backtick,
            '#' => CharKind::Hashtag,
            '@' => CharKind::Address,
            '"' => CharKind::DoubleQuote,
            '\'' => CharKind::SingleQuote,
            _ => CharKind::Operator
        }
    }
}