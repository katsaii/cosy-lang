use crate::error::{ Diagnostic, IssueManager };
use crate::ir::{ ast, hir };

/// Convert an AST into an untyped HIR.
pub fn from_ast(
    issues : &mut IssueManager,
    ast_node : &ast::Node,
) -> hir::Module {
    let mut ctx = Ast2Hir { issues };
    ctx.lower_module(ast_node)
}

struct Ast2Hir<'a> {
    issues : &'a mut IssueManager,
}

impl<'a> Ast2Hir<'a> {
    fn assert(&mut self, got : &ast::Node, message : &str) -> Option<()> {
        Diagnostic::error()
            .message(("malformed AST! {}, got {}", [
                message.into(), got.name().into()
            ]))
            .label(got.primary_location())
            .report(self.issues);
        None
    }

    fn lower_module(&mut self, ast_node : &ast::Node) -> hir::Module {
        hir::Module::default()
    }
}

/*
pub mod hir;
pub mod casm;

use crate::parse::ast;
use crate::error::{ IssueManager, Diagnostic };

/// Responsible for building the first-pass HIR of a cosy file from its AST.
pub struct Ast2Hir<'a> {
    issues : &'a mut IssueManager,
}

impl<'a> Ast2Hir<'a> {
    /// Lowers the AST into HIR, adding simple type information and writing errors
    /// to `issues`.
    pub fn lower(
        issues : &'a mut IssueManager,
        ast_node : &ast::Node
    ) -> hir::Module {
        let mut ctx = Self { issues };
        ctx.lower_module(ast_node)
    }

    fn assert(&mut self, got : &ast::Node, message : &str) -> Option<()> {
        Diagnostic::error()
            .message(("malformed AST! {}, got {}", [message.into(), got.name().into()]))
            .label(got.primary_location())
            .report(self.issues);
        None
    }

    fn lower_module(&mut self, ast_node : &ast::Node) -> hir::Module {
        let items = Vec::new();
        //self.assert(ast_node, "expected block");
        hir::Module { items }
    }
}

/// Responsible for converting HIR into Cosy ASM.
pub struct Hir2Casm<'a> {
    issues : &'a mut IssueManager,
}

impl<'a> Hir2Casm<'a> {
    /// Lowers a collection of HIR into Cosy ASM, resolving inter-module symbol
    /// linking.
    ///
    /// Writes any errors to `issues`.
    pub fn lower(
        issues : &'a mut IssueManager,
        hir : &hir::Module
    ) -> casm::Package {
        let mut ctx = Self { issues };
        ctx.lower_module(hir)
    }

    fn lower_module(&mut self, _hir : &hir::Module) -> casm::Package {
        casm::Package
    }
}
*/