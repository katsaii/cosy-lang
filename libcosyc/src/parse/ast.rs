use bincode::{ Encode, Decode };

use crate::source::Located;

/// Declaration visibility level.
#[derive(Debug, Encode, Decode)]
pub enum Visibility {
    Public,
    Internal,
}

/// An owned section of source code, such as a string literal after resolving
/// escape codes.
pub type Symbol = String;

/// All AST nodes available to Cosy.
///
/// Although it's possible to construct them, any malformed ASTs will raise an
/// error during the AST -> HIR lowering step.
#[derive(Debug, Encode, Decode)]
pub enum Node {
    // expressions
    NumIntegral(Located<u128>),
    NumRational(Located<Symbol>),
    Bool(Located<bool>),
    Id(Located<Symbol>),
    Block(Located<Vec<Node>>),
    Parens(Located<Box<Node>>),
    // statments
    Local {
        name : Located<Symbol>,
        init : Option<Box<Node>>,
    },
    // declarations
    Fn {
        name : Located<Symbol>,
        body : Box<Node>,
    },
    // misc
    Scope {
        vis : Located<Visibility>,
        node : Box<Node>,
    },
}