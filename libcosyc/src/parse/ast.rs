use crate::source::{ Symbol, Location };

/// All expressions available to Cosy. Note: this doesn't include constructs
/// like `var`, since those are statements.
#[derive(Debug)]
pub enum ExprKind {
    NumIntegral(u128),
    NumRational(Symbol),
    Bool(bool),
    Id(Symbol),
    Block(Vec<Stmt>),
    Parens(Box<Expr>),
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
    LocalVar {
        name : Symbol,
        init : Option<Expr>,
    },
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

/// A module associates declarations with a name. Modules are hierarchial, and
/// can contain submodules.
#[derive(Debug, Default)]
pub struct Module {
    /// Submodule definitions.
    pub submodules : Vec<(Visibility, Module)>,
    /// Top-level declarations.
    pub decls : Vec<(Visibility, Decl)>,
    /// The name of this module.
    pub name : Symbol,
    /// The location of this module definition, if one exists.
    pub location : Option<Location>,
    pub(super) initialised : bool,
}