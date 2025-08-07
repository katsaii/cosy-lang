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
}

#[derive(Debug)]
pub struct Decl {
    pub kind : DeclKind,
    /// The span of this declaration in the source code.
    pub location : Location,
}

/// Top-level declaration visibility.
pub enum Visibility {
    Public,
    Internal,
}

impl Default for Visibility {
    fn default() -> Self { Visibility::Internal }
}

/// A module associates declarations with a name. Modules are hierarchial, and
/// can contain submodules.
pub struct Module {
    /// Submodules of this module. Often declared on the first line of a
    /// module definition.
    pub submodules : Vec<(Visibility, Module)>,
    /// The name of this module. If the module is a file, this will be the file
    /// name.
    pub name : String,
    /// Top-level declarations 
    pub decls : Vec<(Visibility, Decl)>,
}