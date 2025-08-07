pub mod hir;
pub mod lex;

use crate::source::File;
use crate::error::IssueManager;

/// Parses the contents of a Cosy source file into untyped HIR.
pub struct Parser<'a> {
    issues : &'a mut IssueManager,
    file : &'a File,
    lexer : lex::Lexer<'a>,
}

impl<'a> Parser<'a> {
    /// Parses the given file, reporting any parser errors to `issues`.
    ///
    /// Returns `true` if the file was parsed successfully, and `false` if any
    /// fatal errors occurred.
    pub fn parse(
        issues : &'a mut IssueManager,
        file : &'a File,
        module : &mut hir::Module,
    ) -> bool {
        let lexer = lex::Lexer::new(file.get_src());
        let mut parser = Self { issues, file, lexer };
        true
    }
}