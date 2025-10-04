//! Similar to the AST, except it supports type annoations, and simplifies some
//! language constructs.

use crate::source::Located;

pub use crate::parse::ast::{ Symbol, Visibility };

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
    NumIntegral(Located<u128>),
    NumRational(Located<Symbol>),
    Bool(Located<bool>),
    Id(Located<Symbol>),
    Block(Located<Vec<Stmt>>),
}

/// All statements available to Cosy.
#[derive(Debug)]
pub enum Stmt {
    Decl(Decl),
    Expr(Expr),
    Local {
        name : Located<Symbol>,
        init : Option<Expr>,
    },
}

/// All declarations available to Cosy. Note: these should all be valid
/// top-level declarations.
#[derive(Debug)]
pub enum Decl {
    Fn {
        name : Located<Symbol>,
        body : Box<Expr>,
    },
}

/// Top-level declarations
#[derive(Debug)]
pub struct TopDecl {
    vis : Visibility,
    decl : Decl,
}