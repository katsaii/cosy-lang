use std::collections::HashMap;
use crate::source::{ Symbol, Location };

#[derive(Debug)]
pub enum NodeKind {
    NumIntegral(u128),
    NumRational(Symbol),
    Bool(bool),
    Id(Symbol),
    Block,
    Assign,
    Fn,
    Module,
}

impl NodeKind {
    /// Creates a new AST node from this node kind.
    pub fn new(
        self,
        location : Location,
        children : impl Into<Vec<Node>>,
    ) -> Node {
        Node {
            kind : self,
            location,
            children : children.into(),
        }
    }
}

/// An AST node encodes purely syntactic information.
///
/// Although it's possible to create malformed ASTs, only well-formed ASTs will
/// be accepted by the AST -> HIR converter.
#[derive(Debug)]
pub struct Node {
    pub kind : NodeKind,
    /// The span of this AST node in the source code.
    pub location : Location,
    /// Descendants of this AST node, e.g. parameters to a function.
    pub children : Vec<Node>,
}