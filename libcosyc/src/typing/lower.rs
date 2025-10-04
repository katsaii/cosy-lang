use crate::{ parse::ast, typing::hir };
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
        self.assert(ast_node, "expected block");
        hir::Module { items }
    }
}