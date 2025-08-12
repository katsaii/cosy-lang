//! Similar to the AST, except it supports type annoations, and simplifies some
//! language constructs.

pub use crate::parse::ast::Visibility;

use std::collections::HashMap;
use crate::source::{ Symbol, Location };
use crate::typing::TypeId;

pub type LocalId = usize;

/// See [`crate::parse::ast::ExprKind`].
#[derive(Debug)]
pub enum ExprKind {
    NumIntegral(u128),
    NumRational(Symbol),
    Bool(bool),
    LocalId(LocalId),
    Block {
        locals : Vec<LocalId>,
        stmts : Vec<Stmt>,
        result : Box<Expr>
    },
}

#[derive(Debug)]
pub struct Expr {
    pub kind : ExprKind,
    /// The span of this expression in the source code.
    pub location : Location,
    /// The type of this expression.
    pub ty_var : TypeId,
}

/// See [`crate::parse::ast::StmtKind`].
#[derive(Debug)]
pub enum StmtKind {
    Decl(Decl),
    Expr(Expr),
    Assign(Expr, Expr),
}

#[derive(Debug)]
pub struct Stmt {
    pub kind : StmtKind,
    /// The span of this statement in the source code.
    pub location : Location,
}

/// See [`crate::parse::ast::DeclKind`].
#[derive(Debug)]
pub enum DeclKind {
    Fn {
        name : Symbol,
        body : Expr,
    },
}

#[derive(Debug)]
pub struct Decl {
    pub kind : DeclKind,
    /// The span of this declaration in the source code.
    pub location : Location,
}

#[derive(Debug)]
pub struct SubModule {
    pub visibility : Visibility,
    pub module : Module,
    /// The location of this submodules name in the source code.
    pub location : Location,
}

/// A module associates declarations with a name. Modules are hierarchial, and
/// can contain submodules.
#[derive(Debug, Default)]
pub struct Module {
    /// A map from submodule names to submodule definitions.
    pub submodules : HashMap<Symbol, SubModule>,
    /// Top-level declarations 
    pub decls : Vec<(Visibility, Decl)>,
}