use crate::source::{ Symbol, Location };

/// Declaration visibility level.
#[derive(Debug)]
pub enum Visibility {
    Public,
    Internal,
}

/// All AST nodes available to Cosy.
///
/// Although it's possible to construct them, any malformed ASTs will raise an
/// error during the AST -> HIR lowering step.
#[derive(Debug)]
pub enum NodeKind {
    // expressions
    NumIntegral(u128),
    NumRational(Symbol),
    Bool(bool),
    Id(Symbol),
    Block(Vec<Node>),
    Parens(Box<Node>),
    // statments
    Local {
        name : Symbol,
        init : Option<Box<Node>>,
    },
    // declarations
    Fn {
        name : Symbol,
        body : Box<Node>,
    },
    Mod {
        name : Symbol,
        body : Box<Node>,
    },
    // misc
    Scope {
        vis : Visibility,
        node : Box<Node>,
    }
}

#[derive(Debug)]
pub struct Node {
    pub kind : NodeKind,
    /// The span of this expression in the source code.
    pub location : Location,
}

pub struct Package {
    /// The name of this package, usually the name of the file that contains the
    /// root module.
    pub name : Symbol,
    pub root : Node,
}