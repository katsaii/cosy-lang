use std::fmt;

/// Cosy semantic token types. Encodes most information about the concrete
/// representation of the source file, such as whether a string is missing
/// a quote.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Token {
    // identifiers
    Id,
    IdRaw { unclosed : bool },
    // literals
    NumIntegral,
    NumRational,
    NumRadix,
    Bool(bool),
    // symbols
    LParen,
    RParen,
    LBox,
    RBox,
    LBrace,
    RBrace,
    Equal,
    Colon,
    ColonColon,
    Dot,
    Comma,
    // keywords
    Do,
    End,
    Else,
    Local,
    Fn,
    Mod,
    Pub,
    Where,
    // miscellaneous
    Comment,
    LineBreak { implicit : bool },
    LineContinue,
    EoF,
    Unknown(char),
}

impl Token {
    pub(crate) fn to_str(&self) -> &'static str {
        match self {
            | Token::Id
            | Token::IdRaw { .. } => "id",
            | Token::NumIntegral
            | Token::NumRational
            | Token::NumRadix => "number",
            Token::Bool(..) => "bool",
            Token::LParen => "`(`",
            Token::RParen => "`)`",
            Token::LBox => "`[`",
            Token::RBox => "`]`",
            Token::LBrace => "`{`",
            Token::RBrace => "`}`",
            Token::Equal => "`=`",
            Token::Colon => "`:`",
            Token::ColonColon => "`::`",
            Token::Dot => "`.`",
            Token::Comma => "`,`",
            Token::Do => "`do`",
            Token::End  => "`end`",
            Token::Else  => "`else`",
            Token::Local => "`local`",
            Token::Fn => "`fn`",
            Token::Mod => "`mod`",
            Token::Pub => "`pub`",
            Token::Where => "`where`",
            Token::Comment => "comment",
            Token::LineBreak { implicit } => if *implicit { "new line" } else { "`;;`" },
            Token::LineContinue => "`...`",
            Token::EoF => "end of file",
            Token::Unknown(..) => "unexpected char",
        }
    }

    pub(crate) fn from_keyword(lexeme : &str) -> Token {
        match lexeme {
            "do" => Token::Do,
            "end" => Token::End,
            "else" => Token::Else,
            "local" => Token::Local,
            "fn" => Token::Fn,
            "mod" => Token::Mod,
            "pub" => Token::Pub,
            "where" => Token::Where,
            "true" => Token::Bool(true),
            "false" => Token::Bool(false),
            _ => Token::Id,
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f : &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_str())
    }
}