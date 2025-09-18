use std::fmt;
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

/// Pairs a value with its location the source code.
pub struct SourceRef<T> {
    pub value : T,
    pub loc : Location,
}

impl<T : fmt::Debug> fmt::Debug for SourceRef<T> {
    fn fmt(&self, out : &mut fmt::Formatter) -> fmt::Result {
        self.value.fmt(out)
    }
}

/// Stores information about a package, such as is name and modules.
#[derive(Debug)]
pub struct Package {
    pub name : Symbol,
    pub root : Node,
}