//! Similar to the AST, except it performs simple type inference and simplifies
//! some language constructs.

pub mod lower;

use std::io;
use bincode;

use crate::src::{ SourceMap, Located };
use crate::pretty::{ PrettyPrinter, Colour, Decoration };

pub use crate::ir::ast::{ Symbol, Visibility };

#[derive(Debug, Default, bincode::Encode, bincode::Decode)]
pub struct Module {
    pub items : Vec<ModuleItem>,
}

/// Top-level declarations.
#[derive(Debug, bincode::Encode, bincode::Decode)]
pub struct ModuleItem {
    vis : Visibility,
    decl : Decl,
}

/// All expressions available to Cosy. Note: this doesn't include constructs
/// like `var`, since those are statements.
#[derive(Debug, bincode::Encode, bincode::Decode)]
pub enum Expr {
    NumIntegral(Located<u128>),
    NumRational(Located<Symbol>),
    Bool(Located<bool>),
    Id(Located<Symbol>),
    Block(Located<Vec<Stmt>>),
}

/// All statements available to Cosy.
#[derive(Debug, bincode::Encode, bincode::Decode)]
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
#[derive(Debug, bincode::Encode, bincode::Decode)]
pub enum Decl {
    Fn {
        name : Located<Symbol>,
        body : Box<Expr>,
    },
}

/// Pretty prints Cosy HIR for debugging purposes.
pub fn debug_write_hir<W : io::Write>(
    printer : &mut PrettyPrinter<W>,
    _files : &SourceMap,
    module : &Module,
) -> io::Result<()> {
    printer.write(&format!("{:?}", module))?;
    Ok(())
}