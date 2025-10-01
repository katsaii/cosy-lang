use bincode::{ Encode, Decode };

use crate::source::{ Symbol, SourceRef };

/// Declaration visibility level.
#[derive(Debug, Encode, Decode)]
pub enum Visibility {
    Public,
    Internal,
}

/// All AST nodes available to Cosy.
///
/// Although it's possible to construct them, any malformed ASTs will raise an
/// error during the AST -> HIR lowering step.
#[derive(Debug, Encode, Decode)]
pub enum Node {
    // expressions
    NumIntegral(SourceRef<u128>),
    NumRational(SourceRef<Symbol>),
    Bool(SourceRef<bool>),
    Id(SourceRef<Symbol>),
    Block(SourceRef<Vec<Node>>),
    Parens(SourceRef<Box<Node>>),
    // statments
    Local {
        name : SourceRef<Symbol>,
        init : Option<Box<Node>>,
    },
    // declarations
    Fn {
        name : SourceRef<Symbol>,
        body : Box<Node>,
    },
    // misc
    Scope {
        vis : SourceRef<Visibility>,
        node : Box<Node>,
    },
}