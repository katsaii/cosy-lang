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
    Nothing,
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
    Begin,
    End,
    Local,
    Fn,
    Module,
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
            Self::Nothing => "`nothing`",
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
            Self::Begin => "`begin`",
            Self::End  => "`end`",
            Self::Local => "`local`",
            Self::Fn => "`fn`",
            Self::Module => "`module`",
            Self::Comment => "comment",
            Self::LineBreak { implicit } => if *implicit { "newline" } else { "`;;`" },
            Self::LineContinue => "`...`",
            Self::EoF => "end of file",
            Self::Unknown(..) => "unexpected char",
        }
    }
}

impl From<Token> for TextFragment {
    fn from(token : Token) -> TextFragment {
        TextFragment::Text(token.to_str().to_string())
    }
}