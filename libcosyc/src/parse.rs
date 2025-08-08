pub mod hir;
pub mod lex;

use std::result;
use lex::Token;
use crate::source::{ Location, File, Span, Symbol };
use crate::error::{ IssueManager, Diagnostic };

type Result<T> = result::Result<T, ()>;

/// Parses the contents of a Cosy source file into untyped HIR.
pub struct Parser<'a> {
    issues : &'a mut IssueManager,
    file : &'a File,
    lexer : lex::Lexer<'a>,
}

impl<'a> Parser<'a> {
    /// Parses a file, writing its generated HIR to the given module.
    ///
    /// Any errors encountered whilst parsing are reported to `issues`.
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
        let is_well_formed = parser.parse_module(module).is_ok();
        is_well_formed && !parser.issues.has_errors()
    }

    fn peek_location(&self) -> Location {
        self.location(self.lexer.peek_span())
    }

    fn location(&self, span : &Span) -> Location {
        self.file.make_location(span)
    }

    fn recover(&mut self) {
        while !matches!(self.lexer.peek(),
            | Token::EoF
            | Token::End
            | Token::Local
            | Token::Fn
            | Token::Module
        ) {
            self.lexer.next();
        }
    }

    fn expect(&mut self, expected : Token) -> Result<lex::TokenSpan> {
        let (span, got) = self.lexer.next();
        if expected == got {
            return Ok((span, got));
        }
        let location = self.file.make_location(&span);
        Diagnostic::error()
            .message(("expected {}, got {}", [expected.into(), got.into()]))
            .label(location)
            .report(self.issues);
        Err(())
    }

    fn parse_module(&mut self, module : &mut hir::Module) -> Result<()> {
        while !matches!(self.lexer.peek(), Token::EoF) {
            let visibility = hir::Visibility::default();
            if let Some(result) = flob(self.parse_decl()) {
                if let Ok(decl) = result {
                    module.decls.push((visibility, decl));
                } else {
                    self.recover();
                }
            } else {
                self.expect(Token::Module)?;
                let location = self.peek_location();
                if let Ok(id) = self.parse_id() {
                    module.submodules.insert(id, hir::SubModule {
                        location,
                        module : hir::Module::default(),
                        visibility
                    });
                } else {
                    self.recover();
                }
            }
        }
        Ok(())
    }

    fn parse_decl(&mut self) -> Result<Option<hir::Decl>> {
        let decl = if matches!(self.lexer.peek(), Token::Fn) {
            self.lexer.next();
            let location = self.peek_location();
            let name = self.parse_id()?;
            self.expect(Token::LParen)?;
            self.expect(Token::RParen)?;
            let body = self.parse_expr()?;
            hir::Decl {
                location,
                kind : hir::DeclKind::Fn { name, body }
            }
        } else {
            return Ok(None);
        };
        Ok(Some(decl))
    }

    fn parse_expr(&mut self) -> Result<hir::Expr> {
        let location = self.peek_location();
        Ok(hir::Expr {
            location,
            kind : hir::ExprKind::Bool(true),
            ty_var : 0,
        })
    }

    fn parse_id(&mut self) -> Result<Symbol> {
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
}

fn flob<T, E>(x : result::Result<Option<T>, E>) -> Option<result::Result<T, E>> {
    match x {
        Ok(Some(x)) => Some(Ok(x)),
        Ok(None) => None,
        Err(err) => Some(Err(err)),
    }
}