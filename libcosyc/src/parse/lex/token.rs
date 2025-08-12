use crate::error::TextFragment;

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
    // miscellaneous
    Comment,
    LineBreak { implicit : bool },
    LineContinue,
    EoF,
    Unknown(char),
}

impl Token {
    fn to_str(&self) -> &'static str {
        match self {
            | Self::Id
            | Self::IdRaw { .. } => "id",
            | Self::NumIntegral
            | Self::NumRational
            | Self::NumRadix => "number",
            Self::Bool(..) => "bool",
            Self::LParen => "`(`",
            Self::RParen => "`)`",
            Self::LBox => "`[`",
            Self::RBox => "`]`",
            Self::LBrace => "`{`",
            Self::RBrace => "`}`",
            Self::Equal => "`=`",
            Self::Colon => "`:`",
            Self::ColonColon => "`::`",
            Self::Dot => "`.`",
            Self::Comma => "`,`",
            Self::Do => "`do`",
            Self::End  => "`end`",
            Self::Else  => "`else`",
            Self::Local => "`local`",
            Self::Fn => "`fn`",
            Self::Mod => "`mod`",
            Self::Comment => "comment",
            Self::LineBreak { implicit } => if *implicit { "new line" } else { "`;;`" },
            Self::LineContinue => "`...`",
            Self::EoF => "end of file",
            Self::Unknown(..) => "unexpected char",
        }
    }

    /// Parses a token from an alphanumeric lexeme.
    pub fn from_lexeme(lexeme : &str) -> Self {
        match lexeme {
            "do" => Self::Do,
            "end" => Self::End,
            "else" => Self::Else,
            "local" => Self::Local,
            "fn" => Self::Fn,
            "mod" => Self::Mod,
            "true" => Self::Bool(true),
            "false" => Self::Bool(false),
            _ => Self::Id,
        }
    }
}

impl From<Token> for TextFragment {
    fn from(token : Token) -> TextFragment {
        TextFragment::Text(token.to_str().to_string())
    }
}