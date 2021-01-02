pub mod syntax;

use libcosyc_diagnostic::{
    source::Span,
    error::{ IssueTracker, CompilerError, ErrorLevel }
};
use libcosyc_scan::{ Lexer, token::TokenKind };
use crate::syntax as ast;
use std::mem;

fn generate_token(lexer : &mut Lexer) -> TokenKind {
    loop {
        // ignore comment tokens
        let token = lexer.generate_token();
        if !matches!(token, TokenKind::Comment) {
            break token;
        }
    }
}

/// Produces a concrete syntax tree from concrete syntax.
pub struct Parser<'a> {
    issues : &'a mut IssueTracker,
    lexer : Lexer<'a>,
    peeked : TokenKind,
    span_previous : Span
}

impl<'a> Parser<'a> {
    /// Creates a new parser from this lexer and issue tracker.
    pub fn new(mut lexer : Lexer<'a>, issues : &'a mut IssueTracker) -> Self {
        let peeked = generate_token(&mut lexer);
        let span_previous = Span::default();
        Self { issues, lexer, peeked, span_previous }
    }

    /// Returns the span of the current lexeme.
    pub fn span(&self) -> &Span {
        self.lexer.span()
    }

    /// Returns the span of the previous lexeme.
    pub fn span_previous(&self) -> &Span {
        &self.span_previous
    }

    /// Returns whether the current token satisfies a predicate `p`.
    /// The function will always return `false` for the EoF token.
    pub fn sat(&self, p : fn(&TokenKind) -> bool) -> bool {
        match &self.peeked {
            TokenKind::EoF => false,
            x => p(x)
        }
    }

    /// Advances the parser and returns the the previous lexeme.
    pub fn advance(&mut self) -> TokenKind {
        self.span_previous = self.lexer.span().clone();
        let next = generate_token(&mut self.lexer);
        mem::replace(&mut self.peeked, next)
    }

    /// Entry point for parsing any expression.
    pub fn parse_expr(&mut self) -> Option<ast::Expr> {
        unimplemented!()
    }
}

/// Generates the AST of this source code and reports any errors to this `IssueTracker`.
pub fn build_ast(src : &str, issues : &mut IssueTracker) -> Option<ast::Expr> {
    let lexer = Lexer::from(src);
    let mut parser = Parser::new(lexer, issues);
    parser.parse_expr()
}
