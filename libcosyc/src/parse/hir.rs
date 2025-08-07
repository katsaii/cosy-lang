use crate::source::{ Symbol, Location };
use crate::typing::TypeId;

/// Local variables become numeric ids in the parser.
pub type LocalId = usize;

/// All expressions available to Cosy. Note: this doesn't include constructs
/// like `var`, since those are statements.
#[derive(Debug)]
pub enum ExprKind {
    NumIntegral(u128),
    NumRational(Symbol),
    Bool(bool),
    LocalId(LocalId),
    Nothing,
    Block {
        locals : Vec<LocalId>,
        stmts : Vec<Stmt>,
        result : Expr
    },
}

#[derive(Debug)]
pub struct Expr {
    pub kind : ExprKind,
    /// The span of this expression in the source code.
    pub location : Location,
    /// The type of this expression.
    //pub ty_var : TypeId,
}

/// All statements available to Cosy.
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

/// All declarations available to Cosy. Note: these should all be valid
/// top-level declarations.
#[derive(Debug)]
pub enum DeclKind {
    Fn {
        name : Symbol,
        body : Expr,
    },
    Module(Symbol),
}

#[derive(Debug)]
pub struct Decl {
    pub kind : DeclKind,
    /// The span of this declaration in the source code.
    pub location : Location,
    /// Whether this declaration is public.
    pub is_public : bool,
}