pub mod ast;
pub mod lex;

use lex::Token;
use crate::source::File;
use crate::error::{ IssueManager, Diagnostic };

macro_rules! expect {
    ($self:expr, $p:pat) => (match $self.lexer.peek() {
        $p => Some($self.lexer.next()),
        _ => None,
    });
}

macro_rules! unterminated {
    ($self:expr, $p:pat) => (match $self.lexer.peek() {
        $p => false,
        x => true,
    });
}

/// Parses the contents of a Cosy source file into untyped AST.
pub struct Parser<'a> {
    issues : &'a mut IssueManager,
    file : &'a File,
    lexer : lex::Lexer<'a>,
}

impl<'a> Parser<'a> {
    /// Parses a file, returning its generated AST nodes as a `Vec`.
    ///
    /// Any errors encountered whilst parsing are reported to `issues`.
    pub fn parse(
        issues : &'a mut IssueManager,
        file : &'a File,
    ) -> Vec<ast::Node> {
        let lexer = lex::Lexer::new(file.get_src());
        let mut parser = Self { issues, file, lexer };
        parser.parse_module_body()
    }

    fn recover(&mut self) {
        while unterminated!(self,
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

    fn parse_module_body(&mut self) -> Vec<ast::Node> {
        let mut nodes = Vec::new();
        while unterminated!(self, Token::EoF | Token::End) {
            if let Some(result) = self.try_parse_decl() {
                if let Some(decl) = result {
                    nodes.push(decl);
                } else {
                    self.recover();
                }
            } else {
                self.assert("invalid definition");
            }
        }
        nodes
    }

    fn try_parse_decl(&mut self) -> Option<Option<ast::Node>> {
        let node = if let Some((span, _)) = expect!(self, Token::Fn) {
            Diagnostic::unimplemented()
                .message("functions")
                .label(self.file.location(&span))
                .report(self.issues);
            None
        } else if let Some((span, _)) = expect!(self, Token::Mod) {
            Diagnostic::unimplemented()
                .message("modules")
                .label(self.file.location(&span))
                .report(self.issues);
            None
        } else {
            return None;
        };
        Some(node)
    }

    /*
    fn parse_module(&mut self, module : &mut ast::Mod) -> Option<()> {
        while !matches!(self.lexer.peek(), Token::EoF) {
            let visibility = ast::Visibility::default();
            if let Some(option) = flob(self.parse_decl()) {
                if let Ok(decl) = option {
                    module.decls.push((visibility, decl));
                } else {
                    self.recover();
                }
            } else {
                self.expect(Token::Mod)?;
                let location = self.peek_location();
                if let Ok(id) = self.parse_id() {
                    module.submodules.insert(id, ast::SubModule {
                        location,
                        module : ast::Mod::default(),
                        visibility
                    });
                } else {
                    self.recover();
                }
            }
        }
        Ok(())
    }

    fn parse_decl(&mut self) -> Option<Option<ast::Decl>> {
        let decl = if matches!(self.lexer.peek(), Token::Fn) {
            self.lexer.next();
            let location = self.peek_location();
            let name = self.parse_id()?;
            self.expect(Token::LParen)?;
            self.expect(Token::RParen)?;
            let body = self.parse_expr()?;
            ast::Decl {
                location,
                kind : ast::DeclKind::Fn { name, body }
            }
        } else {
            return Ok(None);
        };
        Ok(Some(decl))
    }

    fn parse_expr(&mut self) -> Option<ast::Expr> {
        self.parse_expr_terminal()
    }

    fn parse_expr_terminal(&mut self) -> Option<ast::Expr> {
        let expr = if let Token::Bool(val) = self.lexer.peek() {
            let (span, _) = self.lexer.next();
            ast::Expr {
                location : self.file.location(&span),
                kind : ast::NodeKind::Bool(true),
            }
        } else {
            return Err(())
        };
        Ok(expr)
    }

    fn parse_id(&mut self) -> Option<Symbol> {
        let span = if let Token::IdRaw { unclosed } = self.lexer.peek() {
            if *unclosed {
                //let location = 
                //Diagnostic::error()
                //    .label()
                return Err(());
            }
            self.lexer.peek_span().shrink(1, 1)
        } else {
            let (span, _) = self.expect(Token::Id)?;
            span
        };
        Ok(self.lexer.slice(&span).to_string())
    }
    */
}