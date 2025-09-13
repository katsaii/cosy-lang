pub mod ast;
pub mod lex;

use std::path::PathBuf;
use lex::Token;
use crate::source::{ Location, File, FileManager };
use crate::error::{ IssueManager, Diagnostic };

/// Parses the contents of a Cosy source file into untyped AST.
pub struct Parser<'a> {
    issues : &'a mut IssueManager,
    file : &'a File,
    lexer : lex::Lexer<'a>,
}

impl<'a> Parser<'a> {
    /// Parses a file, into the given module.
    ///
    /// Any errors encountered whilst parsing are reported to `issues`.
    pub fn parse(
        issues : &'a mut IssueManager,
        file : &'a File,
        //module : &mut ast::Node,
    ) {
        let lexer = lex::Lexer::new(file.get_src());
        let mut parser = Self { issues, file, lexer };
        //parser.parse_module_body(module);
    }
/*
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

    fn parse_module_body(
        &mut self,
        module : &mut ast::Module,
    ) -> Option<()> {
        module.initialised = true;
        while !matches!(self.lexer.peek(), Token::End | Token::EoF) {
            let visibility = if let Token::Pub = self.lexer.peek() {
                self.lexer.next();
                ast::Visibility::Public
            } else {
                ast::Visibility::Internal
            };
            if let Token::Mod = self.lexer.peek() {
                self.lexer.next();
                let (location, name) = self.parse_id()?;
                let mut submodule = ast::Module {
                    name, location : Some(location), ..ast::Module::default()
                };
                if let Token::Where = self.lexer.peek() {
                    self.lexer.next();
                    self.parse_module_body(&mut submodule);
                    self.assert_token(Token::End);
                }
                module.submodules.push((visibility, submodule));
            } else if let Some(result) = self.try_parse_decl() {
                if let Some(decl) = result {
                    module.decls.push((visibility, decl));
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

    fn parse_stmt(&mut self) -> Option<ast::Stmt> {
        if let Some(decl) = self.try_parse_decl() {
            let decl = decl?;
            Some(ast::Stmt {
                location : decl.location,
                kind : ast::StmtKind::Decl(decl),
            })
        } else if let Token::Local = self.lexer.peek() {
            self.lexer.next();
            let (location, name) = self.parse_id()?;
            let init = if let Token::Equal = self.lexer.peek() {
                self.lexer.next();
                Some(self.parse_expr()?)
            } else {
                None
            };
            Some(ast::Stmt {
                location,
                kind : ast::StmtKind::LocalVar { name, init }
            })
        } else {
            let expr = self.parse_expr()?;
            Some(ast::Stmt {
                location : expr.location,
                kind : ast::StmtKind::Expr(expr),
            })
        }
    }

    fn parse_expr(&mut self) -> Option<ast::Expr> {
        self.parse_expr_stmt()
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

    fn parse_expr_block(&mut self) -> Option<ast::Expr> {
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
        let location = self.file.location(&span);
        Some(ast::Expr {
            location,
            kind : ast::ExprKind::Block(stmts),
        })
    }

    fn parse_expr_terminal(&mut self) -> Option<ast::Expr> {
        if let Token::NumIntegral = self.lexer.peek() {
            let (span, _) = self.lexer.next();
            let location = self.file.location(&span);
            let n_string = span.slice(self.file.get_src()).replace("_", "");
            match n_string.parse::<u128>() {
                Ok(n) => Some(ast::Expr {
                    location,
                    kind : ast::ExprKind::NumIntegral(n),
                }),
                Err(err) => {
                    Diagnostic::error()
                        .message(("{}", [err.into()]))
                        .label(location)
                        .report(self.issues);
                    None
                }
            }
        } else if let Token::NumRational = self.lexer.peek() {
            let (span, _) = self.lexer.next();
            let location = self.file.location(&span);
            let n_string = span.slice(self.file.get_src()).replace("_", "");
            Some(ast::Expr {
                location,
                kind : ast::ExprKind::NumRational(n_string),
            })
        } else if let Token::Bool(b) = self.lexer.peek() {
            let b = *b;
            let (span, _) = self.lexer.next();
            let location = self.file.location(&span);
            Some(ast::Expr {
                location,
                kind : ast::ExprKind::Bool(b),
            })
        } else if let Token::LParen = self.lexer.peek() {
            let (span_start, _) = self.lexer.next();
            let expr = self.parse_expr()?;
            let (span_end, _) = self.assert_token(Token::RParen)?;
            let location = self.file.location(&span_start.join(&span_end));
            Some(ast::Expr {
                location,
                kind : ast::ExprKind::Parens(Box::new(expr)),
            })
        } else {
            let (location, name) = self.parse_id()?;
            Some(ast::Expr {
                location,
                kind : ast::ExprKind::Id(name),
            })
        }
    }

    fn parse_id(&mut self) -> Option<(Location, String)> {
        let location = self.file.location(&self.lexer.peek_span());
        let span = if let Token::IdRaw { unclosed } = self.lexer.peek() {
            let unclosed = *unclosed;
            let (span, _) = self.lexer.next();
            if unclosed {
                Diagnostic::error()
                    .message("unclosed raw identifier")
                    .label((location, ["expected a closing '`' here".into()]))
                    .report(self.issues);
                span.shrink(1, 0)
            } else {
                span.shrink(1, 1)
            }
        } else {
            self.assert_token(Token::Id)?.0
        };
        Some((location, self.lexer.slice(&span).to_string()))
    }
*/
}

fn open_file<'a>(
    issues : &mut IssueManager,
    files : &'a mut FileManager,
    file_path : PathBuf,
    location : Option<Location>,
) -> Option<&'a File> {
/*
    match files.load(file_path) {
        Ok(file_id) => Some(files.get_file(file_id)),
        Err(err) => {
            let mut diag = Diagnostic::from(err);
            if let Some(location) = location {
                diag = diag.label((location, [
                    "module defined here".into()
                ]));
            }
            diag.report(issues);
            None
        },
    }
*/
    None
}

/// Parses a package from a root module. Also handles the recursive parsing
/// of submodules.
///
/// Any errors encountered whilst parsing are reported to `issues`.
pub fn from_file(
    issues : &mut IssueManager,
    files : &mut FileManager,
    file_path : PathBuf,
) -> Option<ast::Node> {
/*
    let file = open_file(issues, files, file_path, None)?;
    let name = file.path.file_stem().unwrap().to_string_lossy().to_string();
    if name.chars().any(char::is_whitespace) {
        Diagnostic::error()
            .message(("package name '{}' should not contain whitespace", [
                name.clone().into(),
            ]))
            .report(issues);
    }
    let mut module = ast::Module { name, ..ast::Module::default() };
    let mut module_root_dir = file.path.parent().unwrap().to_path_buf();
    Parser::parse(issues, file, &mut module);
    from_file_submodules(issues, files, &mut module, &mut module_root_dir);
    Some(module)
*/
    None
}

/*
fn from_file_submodules(
    issues : &mut IssueManager,
    files : &mut FileManager,
    module : &mut ast::Module,
    module_root_dir : &mut PathBuf,
) -> Option<()> {
    module_root_dir.push(&module.name);
    for (_, submodule) in module.submodules.iter_mut() {
        if !submodule.initialised {
            let mut file_path = module_root_dir.clone();
            file_path.push(&submodule.name);
            file_path.set_extension("cy");
            let file = if let Some(x) = open_file(
                issues, files, file_path, submodule.location
            ) { x } else {
                continue;
            };
            Parser::parse(issues, file, submodule);
        }
        from_file_submodules(issues, files, submodule, module_root_dir);
    }
    module_root_dir.pop();
    Some(())
}
*/