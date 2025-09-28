//! Similar to the AST, except it supports type annoations, and simplifies some
//! language constructs.

pub use crate::parse::ast::Visibility;

use crate::parse::ast;
use crate::source::{ Symbol, Location, SourceRef };

/// HIR packages erase the concept of multiple-files, collapsing them into a
/// single translation unit.
#[derive(Debug)]
pub struct Package {
    pub name : String,
    pub decls : Vec<TopDecl>,
}

/// All expressions available to Cosy. Note: this doesn't include constructs
/// like `var`, since those are statements.
#[derive(Debug)]
pub enum Expr {
    NumIntegral(SourceRef<u128>),
    NumRational(SourceRef<Symbol>),
    Bool(SourceRef<bool>),
    Id(SourceRef<Symbol>),
    Block(SourceRef<Vec<Stmt>>),
}

/// All statements available to Cosy.
#[derive(Debug)]
pub enum Stmt {
    Decl(Decl),
    Expr(Expr),
    Local {
        name : SourceRef<Symbol>,
        init : Option<Expr>,
    },
}

/// All declarations available to Cosy. Note: these should all be valid
/// top-level declarations.
#[derive(Debug)]
pub enum Decl {
    Fn {
        name : SourceRef<Symbol>,
        body : Box<Expr>,
    },
}

/// Top-level declarations
#[derive(Debug)]
pub struct TopDecl {
    vis : Visibility,
    decl : Decl,
}