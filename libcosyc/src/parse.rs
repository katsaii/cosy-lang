pub mod ast;
pub mod lex;

use lex::Token;
use crate::source::{ Location, File };
use crate::error::{ IssueManager, Diagnostic };

/// Parses the contents of a Cosy source file into untyped AST.
pub struct Parser<'a> {
    issues : &'a mut IssueManager,
    file : &'a File,
    lexer : lex::Lexer<'a>,
}

impl<'a> Parser<'a> {
    /// Parses a file, into the given module. Returns a `Vec` of any unparsed
    /// submodules.
    ///
    /// Any errors encountered whilst parsing are reported to `issues`.
    pub fn parse(
        issues : &'a mut IssueManager,
        file : &'a File,
        module : &mut ast::Module,
    ) -> () {
        let lexer = lex::Lexer::new(file.get_src());
        let mut parser = Self { issues, file, lexer };
        parser.parse_module_body(module);
    }

    fn recover(&mut self) {
        while !matches!(self.lexer.peek(),
            | Token::EoF
            | Token::End
            | Token::Local
            | Token::Fn
            | Token::Mod
        ) {
            self.lexer.next();
        }
    }

    fn assert_token(&mut self, expected : Token) -> Option<lex::TokenSpan> {
        let (span, got) = self.lexer.next();
        if expected == got {
            return Some((span, got));
        }
        Diagnostic::error()
            .message(("expected {}, got {}", [expected.into(), got.into()]))
            .label(self.file.location(&span))
            .report(self.issues);
        None
    }

    fn assert(&mut self, message : &str) -> Option<()> {
        let (span, got) = self.lexer.next();
        Diagnostic::error()
            .message(("{}, got {}", [message.into(), got.into()]))
            .label(self.file.location(&span))
            .report(self.issues);
        None
    }

    fn parse_module_body(
        &mut self,
        module : &mut ast::Module,
    ) -> Option<()> {
        while !matches!(self.lexer.peek(), Token::EoF | Token::End) {
            let visibility = ast::Visibility::Public;
            if let Token::Mod = self.lexer.peek() {
                let span = self.lexer.next().0;
                let location = self.file.location(&span);
                Diagnostic::unimplemented()
                    .message("modules")
                    .label(location)
                    .report(self.issues);
            } else if let Some(result) = self.try_parse_decl() {
                if let Some(decl) = result {
                    module.decls.push(ast::TopDecl { visibility, decl });
                } else {
                    self.recover();
                }
            } else {
                self.assert("unexpected symbol in declaration scope");
            }
        }
        Some(())
    }

    fn try_parse_decl(&mut self) -> Option<Option<ast::Decl>> {
        let node = if let Token::Fn = self.lexer.peek() {
            self.lexer.next();
            // get function signature
            let (location, name) = self.parse_id()?;
            self.assert_token(Token::LParen)?;
            self.assert_token(Token::RParen)?;
            // get function body
            self.assert_token(Token::Do)?;
            let body = self.parse_expr_block()?;
            self.assert_token(Token::End)?;
            Some(ast::Decl {
                location,
                kind : ast::DeclKind::Fn { name, body }
            })
        } else {
            return None;
        };
        Some(node)
    }

    fn parse_expr(&mut self) -> Option<ast::Expr> {
        self.parse_expr_block()
    }

    fn parse_expr_block(&mut self) -> Option<ast::Expr> {
        let span = self.lexer.next().0;
        let location = self.file.location(&span);
        Diagnostic::unimplemented()
            .message("blocks")
            .label(location)
            .report(self.issues);
        None
    }

    fn parse_expr_stmt(&mut self) -> Option<ast::Expr> {
        if let Token::Do = self.lexer.peek() {
            self.lexer.next();
            let expr = self.parse_expr_block()?;
            self.assert_token(Token::End)?;
            Some(expr)
        } else {
            self.parse_expr_terminal()
        }
    }

    fn parse_expr_terminal(&mut self) -> Option<ast::Expr> {
        if let Token::Bool(val) = self.lexer.peek() {
            let val = *val;
            let (span, _) = self.lexer.next();
            let location = self.file.location(&span);
            Some(ast::Expr {
                location,
                kind : ast::ExprKind::Bool(val),
            })
        } else {
            self.assert_token(Token::LParen)?;
            let expr = self.parse_expr()?;
            self.assert_token(Token::RParen)?;
            Some(expr)
        }
    }

    fn parse_id(&mut self) -> Option<(Location, String)> {
        let location = self.file.location(&self.lexer.peek_span());
        let span = if let Token::IdRaw { unclosed } = self.lexer.peek() {
            let unclosed = *unclosed;
            let (span, _) = self.lexer.next();
            if unclosed {
                //Diagnostic::error()
                //    .label()
                span.shrink(1, 0)
            } else {
                span.shrink(1, 1)
            }
        } else {
            self.assert_token(Token::Id)?.0
        };
        Some((location, self.lexer.slice(&span).to_string()))
    }
}