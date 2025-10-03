//use crate::parse::ast;
use crate::error::{ IssueManager };

struct ASTLowerCtx<'a> {
    issues : &'a mut IssueManager,
}

impl<'a> ASTLowerCtx<'a> {
    //fn lower_module() -> 
}

/// Lower the AST into HIR, adding simple type information.
pub fn ast_to_hir() {

}