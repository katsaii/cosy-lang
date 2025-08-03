/// Cosy semantic token types. Encodes most information about the concrete
/// representation of the source file, such as whether a string is missing
/// a quote.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    // identifiers
    Id { is_hole : bool },
    IdRaw { unclosed : bool },
    // literals
    NumIntegral,
    NumRational,
    NumRadix,
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
    Var,
    Fn,
    Mod,
    // miscellaneous
    Comment,
    Whitespace,
    LineBreak { implicit : bool },
    LineContinue { dot_count : usize },
    EoF,
    Unknown(char),
}

impl Token {
    /// Returns whether the token is a valid identifier.
    pub fn is_identifier(&self) -> bool {
        matches!(self, Self::Id { .. } | Self::IdRaw { .. })
    }

    /// Returns whether this token is an underscore.
    pub fn is_hole(&self) -> bool {
        matches!(self, Self::Id { is_hole : true })
    }

    /// Returns whether the token is a valid whitespace character.
    pub fn is_whitespace(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Comment)
    }

    /// Returns whether the token is the end-of-file character.
    pub fn is_eof(&self) -> bool {
        matches!(self, Self::EoF)
    }

    /// Returns whether the token is a valid terminator character.
    pub fn is_eol(&self) -> bool {
        matches!(self, Self::LineBreak { .. })
    }

    /// Returns whether this token can terminate the parsing of a block.
    pub fn is_block_terminator(&self) -> bool {
        matches!(self, Self::EoF | Self::End)
    }
}