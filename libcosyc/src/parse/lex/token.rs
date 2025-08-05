/// Cosy semantic token types. Encodes most information about the concrete
/// representation of the source file, such as whether a string is missing
/// a quote.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
    LineBreak { implicit : bool },
    LineContinue,
    EoF,
    Unknown(char),
}