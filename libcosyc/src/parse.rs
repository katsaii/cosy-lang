pub mod hir;
pub mod lex;

use crate::source::File;
use crate::error::IssueManager;

/// Parses the contents of a Cosy source file into untyped HIR.
pub struct Parser<'a, 'hir> {
    issues : &'a mut IssueManager,
    file : &'a File,
    lexer : lex::Lexer<'a>,
    module : &'hir hir::Module,
}

impl<'a, 'hir> Parser<'a, 'hir> {
    /// Parses the given file, writing its generated HIR to the given module.
    ///
    /// Any errors encountered whilst parsing are reported to `issues`.
    ///
    /// Returns `true` if the file was parsed successfully, and `false` if any
    /// fatal errors occurred.
    pub fn parse(
        issues : &'a mut IssueManager,
        file : &'a File,
        module : &'hir mut hir::Module,
    ) -> bool {
        let lexer = lex::Lexer::new(file.get_src());
        let mut parser = Self { issues, file, lexer, module };
        parser.parse_module()
    }

    fn parse_module(&mut self) -> bool {
        true
    }
}