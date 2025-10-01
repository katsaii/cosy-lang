///! Parses the contents of a Cosy source file into untyped AST.

pub mod ast;
pub mod lex;

use std::path::PathBuf;
use lex::Token;
use crate::source::{ Symbol, Span, Location, SourceRef, File, FileManager };
use crate::error::{ IssueManager, Diagnostic };

struct Parser<'a> {
    issues : &'a mut IssueManager,
    file : &'a File,
    lexer : lex::Lexer<'a>,
}

impl<'a> Parser<'a> {
    fn parse(
        issues : &'a mut IssueManager,
        file : &'a File,
    ) -> ast::Node {
        let lexer = lex::Lexer::new(file.get_src());
        let mut parser = Self { issues, file, lexer };
        parser.parse_module_body()
    }

    fn make_dbg<T>(&self, span : &Span, value : T) -> SourceRef<T> {
        SourceRef { value, loc : self.make_loc(span) }
    }

    fn make_loc(&self, span : &Span) -> Location {
        self.file.location(span)
    }

    fn recover(&mut self) {
        while
            !matches!(self.lexer.peek(),
                | Token::End
                | Token::Else
                | Token::Local
                | Token::Fn
                | Token::Mod
                | Token::EoF)
        {
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

    fn parse_module_body(&mut self) -> ast::Node {
        let mut decls = Vec::new();
        let span_start = self.lexer.peek_span().clone();
        while !matches!(self.lexer.peek(), Token::End | Token::EoF) {
            let decl = if let Some(result) = self.try_parse_decl() {
                if let Some(decl) = result {
                    decl
                } else {
                    self.recover();
                    continue;
                }
            } else {
                self.assert("unexpected symbol in declaration scope");
                continue;
            };
            decls.push(decl);
        }
        let span = span_start.join(self.lexer.peek_span());
        ast::Node::Block(self.make_dbg(&span, decls))
    }

    fn try_parse_decl(&mut self) -> Option<Option<ast::Node>> {
        let node = if let Token::Fn = self.lexer.peek() {
            self.lexer.next();
            // get function signature
            let name = self.parse_id()?;
            self.assert_token(Token::LParen)?;
            self.assert_token(Token::RParen)?;
            // get function body
            self.assert_token(Token::Do)?;
            let body = Box::new(self.parse_expr_block()?);
            self.assert_token(Token::End)?;
            Some(ast::Node::Fn { name, body })
        } else {
            return None;
        };
        Some(node)
    }

    fn parse_stmt(&mut self) -> Option<ast::Node> {
        if let Some(decl) = self.try_parse_decl() {
            decl
        } else if let Token::Local = self.lexer.peek() {
            self.lexer.next();
            let name = self.parse_id()?;
            let init = if let Token::Equal = self.lexer.peek() {
                self.lexer.next();
                Some(Box::new(self.parse_expr()?))
            } else {
                None
            };
            Some(ast::Node::Local { name, init })
        } else {
            self.parse_expr()
        }
    }

    fn parse_expr(&mut self) -> Option<ast::Node> {
        self.parse_expr_stmt()
    }

    fn parse_expr_stmt(&mut self) -> Option<ast::Node> {
        if let Token::Do = self.lexer.peek() {
            self.lexer.next();
            let expr = self.parse_expr_block()?;
            self.assert_token(Token::End)?;
            Some(expr)
        } else {
            self.parse_expr_terminal()
        }
    }

    fn parse_expr_block(&mut self) -> Option<ast::Node> {
        let span_start = self.lexer.peek_span().clone();
        let mut stmts = Vec::new();
        while
            !matches!(self.lexer.peek(),
                | Token::End
                | Token::Else
                | Token::EoF)
        {
            if let Some(stmt) = self.parse_stmt() {
                stmts.push(stmt);
            } else {
                self.recover();
            }
        }
        let span = span_start.join(self.lexer.peek_span());
        Some(ast::Node::Block(self.make_dbg(&span, stmts)))
    }

    fn parse_expr_terminal(&mut self) -> Option<ast::Node> {
        if let Token::NumIntegral = self.lexer.peek() {
            let (span, _) = self.lexer.next();
            let n_string = span.slice(self.file.get_src()).replace("_", "");
            match n_string.parse::<u128>() {
                Ok(n) => Some(ast::Node::NumIntegral(self.make_dbg(&span, n))),
                Err(err) => {
                    Diagnostic::error()
                        .message(("{}", [err.into()]))
                        .label(self.file.location(&span))
                        .report(self.issues);
                    None
                }
            }
        } else if let Token::NumRational = self.lexer.peek() {
            let (span, _) = self.lexer.next();
            let n_string = span.slice(self.file.get_src()).replace("_", "");
            Some(ast::Node::NumRational(self.make_dbg(&span, n_string)))
        } else if let Token::Bool(b) = self.lexer.peek() {
            let b = *b;
            let (span, _) = self.lexer.next();
            Some(ast::Node::Bool(self.make_dbg(&span, b)))
        } else if let Token::LParen = self.lexer.peek() {
            let (span_start, _) = self.lexer.next();
            let expr = self.parse_expr()?;
            let (span_end, _) = self.assert_token(Token::RParen)?;
            let span = span_start.join(&span_end);
            Some(ast::Node::Parens(self.make_dbg(&span, Box::new(expr))))
        } else {
            let name = self.parse_id()?;
            Some(ast::Node::Id(name))
        }
    }

    fn parse_id(&mut self) -> Option<SourceRef<Symbol>> {
        let srcloc = if let Token::IdRaw { unclosed } = self.lexer.peek() {
            let unclosed = *unclosed;
            let (span, _) = self.lexer.next();
            let span_inner = if unclosed {
                Diagnostic::error()
                    .message("unclosed raw identifier")
                    .label((self.make_loc(&span), ["expected a closing '`' here".into()]))
                    .report(self.issues);
                span.shrink(1, 0)
            } else {
                span.shrink(1, 1)
            };
            self.make_dbg(&span, self.lexer.slice(&span_inner).to_string())
        } else {
            let span = self.assert_token(Token::Id)?.0;
            self.make_dbg(&span, self.lexer.slice(&span).to_string())
        };
        Some(srcloc)
    }
}

/// Parses a package from a root module. Also handles the recursive parsing
/// of submodules.
///
/// Any errors encountered whilst parsing are reported to `issues`.
pub fn package_from_file(
    issues : &mut IssueManager,
    files : &mut FileManager,
    file_path : PathBuf,
) -> Option<(Symbol, ast::Node)> {
    let file = match files.load(file_path) {
        Ok(file_id) => files.get_file(file_id),
        Err(err) => {
            let diag = Diagnostic::from(err);
            diag.report(issues);
            return None;
        },
    };
    let name = file.path.file_stem().unwrap().to_string_lossy().to_string();
    //let module_dir = file.path.parent().unwrap().to_path_buf();
    let module = Parser::parse(issues, file);
    // TODO :: multiple-files/incremental compilation
    Some((name, module))
}