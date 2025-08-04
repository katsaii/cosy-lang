pub mod log;

use crate::{
    Session, source::FileManager,
    error::{ Diagnostic, IssueStats },
};
use std::io;

/// Any valid diagnostics renderer must satisfy this trait.
pub trait Renderer {
    /// Responsible for for rendering all of the issues reported by a compiler
    /// session.
    fn render_session<W : io::Write>(
        &mut self,
        out : &mut W,
        sess : &Session,
    ) -> io::Result<()> {
        let issues = &sess.issues;
        let files = &sess.files;
        for error in &issues.errors {
            self.render_diagnostic(out, error, files)?;
        }
        if let Some(stats) = issues.error_stats() {
            self.render_stats(out, &stats)?;
        }
        Ok(())
    }

    /// Responsible for for rendering a specific diagnostic.
    fn render_diagnostic<W : io::Write>(
        &mut self,
        out : &mut W,
        error : &Diagnostic,
        files : &FileManager,
    ) -> io::Result<()>;

    /// Responsible for for rendering all of the issues reported by a compiler
    /// session.
    #[allow(unused_variables)]
    fn render_stats<W : io::Write>(
        &mut self,
        out : &mut W,
        stats : &IssueStats,
    ) -> io::Result<()> { Ok(()) }
}

/// Assists with the pretty printing of diagnostics.
pub struct PrettyPrinter {
    column : usize,
    indent : usize,
    indent_stack : Vec<usize>,
}

impl PrettyPrinter {
    /// Writes a string to 
    pub fn write<W : io::Write>(
        &mut self,
        out : &mut W,
        text : &str
    ) -> io::Result<()> {
        unimplemented!()
    }

    /// Skips the next `n` characters in the output stream.
    pub fn skip<W : io::Write>(
        &mut self,
        out : &mut W,
        n : usize
    ) -> io::Result<()> {
        write!(out, "{}", " ".repeat(n))?;
        self.column += n;
        Ok(())
    }

    /// Pushes the current indentation onto the indentation stack. Affects the
    /// indentation of new lines. Returns the new indentation which was
    /// stashed.
    pub fn indent_stash(&mut self) -> usize {
        self.indent_push(self.column);
        self.indent
    }

    /// Pushes a specific indentation onto the indentation stack.
    pub fn indent_push(&mut self, n : usize) {
        self.indent_stack.push(self.indent);
        self.indent = n;
    }

    /// Pops the current indentation from the indentation stash. No effect if
    /// the stash is empty.
    pub fn indent_pop(&mut self) {
        self.indent = self.indent_stack.pop().unwrap_or_default();
    }
}

fn _fmt(template : &str, args : &[&str]) -> String {
    fn get_arg<'a>(args : &'a [&'a str], pos : usize) -> &'a str {
        args.get(pos).map(|x| x as &str).unwrap_or_default()
    }
    fn write_arg(sb : &mut String, s : &str) {
        sb.push_str(s);
    }
    let mut sb = String::new();
    let mut arg_pos = 0;
    let mut prev = '\0';
    let mut skip_close_paren = false;
    for next in template.chars() {
        match (prev, next) {
            (x@'{', '{') | (x@'}', '}') => sb.push(x),
            ('{', '}') => {
                sb.pop(); // pop the `{` character
                let arg = get_arg(args, arg_pos);
                write_arg(&mut sb, arg);
                arg_pos += 1;
            },
            ('{', '*') => {
                sb.pop(); // pop the `{` character
                for i in arg_pos..args.len() {
                    if i > arg_pos {
                        sb.push_str(", ");
                    }
                    let arg = get_arg(args, i);
                    write_arg(&mut sb, arg);
                }
                skip_close_paren = true;
            },
            (_, x) => {
                if !skip_close_paren || x != '}' {
                    skip_close_paren = false;
                    sb.push(x)
                }
            },
        }
        prev = next;
    }
    sb
}