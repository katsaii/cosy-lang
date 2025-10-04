//! Similar to the AST, except it performs simple type inference and simplifies
//! some language constructs.

use bincode::{ Encode, Decode };

use crate::{ source::Located, vfs::Manifest };

pub use crate::parse::ast::{ Symbol, Visibility };

#[derive(Debug, Default, Encode, Decode)]
pub struct Module {
    pub items : Vec<ModuleItem>,
}

/// Top-level declarations.
#[derive(Debug, Encode, Decode)]
pub struct ModuleItem {
    vis : Visibility,
    decl : Decl,
}

/// All expressions available to Cosy. Note: this doesn't include constructs
/// like `var`, since those are statements.
#[derive(Debug, Encode, Decode)]
pub enum Expr {
    NumIntegral(Located<u128>),
    NumRational(Located<Symbol>),
    Bool(Located<bool>),
    Id(Located<Symbol>),
    Block(Located<Vec<Stmt>>),
}

/// All statements available to Cosy.
#[derive(Debug, Encode, Decode)]
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
#[derive(Debug, Encode, Decode)]
pub enum Decl {
    Fn {
        name : Located<Symbol>,
        body : Box<Expr>,
    },
}

/// Pretty prints HIR for debugging purposes.
pub fn debug_print_hir(_manifest : &Manifest, module : &Module) {
    println!("{:?}", module);
}