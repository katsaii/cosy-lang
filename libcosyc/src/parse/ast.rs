use std::collections::HashMap;
use crate::source::{ Symbol, Location };

/// All expressions available to Cosy. Note: this doesn't include constructs
/// like `var`, since those are statements.
#[derive(Debug)]
pub enum ExprKind {
    NumIntegral(u128),
    NumRational(Symbol),
    Bool(bool),
    Id(Symbol),
    Block {
        stmts : Vec<Stmt>,
        result : Box<Expr>
    },
}

#[derive(Debug)]
pub struct Expr {
    pub kind : ExprKind,
    /// The span of this expression in the source code.
    pub location : Location,
}

/// All statements available to Cosy.
#[derive(Debug)]
pub enum StmtKind {
    Assign(Expr, Expr),
    Decl(Decl),
    Expr(Expr),
}

#[derive(Debug)]
pub struct Stmt {
    pub kind : StmtKind,
    /// The span of this statement in the source code.
    pub location : Location,
}

/// All declarations available to Cosy. Note: these should all be valid
/// top-level declarations.
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

/// Top-level declaration visibility.
#[derive(Debug)]
pub enum Visibility {
    Public,
    Internal,
}

impl Default for Visibility {
    fn default() -> Self { Visibility::Internal }
}

#[derive(Debug)]
pub struct TopDecl {
    pub visibility : Visibility,
    pub decl : Decl,
}

#[derive(Debug)]
pub struct SubModule {
    pub visibility : Visibility,
    pub module : Module,
    /// The location of this submodules name in the source code.
    pub location : Location,
    /// The name of this submodule.
    pub name : Symbol,
}

/// A module associates declarations with a name. Modules are hierarchial, and
/// can contain submodules.
#[derive(Debug, Default)]
pub struct Module {
    /// Submodule definitions.
    pub submodules : Vec<SubModule>,
    /// Top-level declarations.
    pub decls : Vec<TopDecl>,
}