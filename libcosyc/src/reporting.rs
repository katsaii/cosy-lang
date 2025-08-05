pub mod log;

use crate::{
    Session,
    source::FileManager,
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
    do_indent : bool,
}

impl PrettyPrinter {
    /// Writes a string to the output stream, sanitising any escape codes.
    pub fn write<W : io::Write>(
        &mut self,
        out : &mut W,
        text : &str,
    ) -> io::Result<()> {
        self.ensure_indented(out)?;
        let mut chr_prev = '\0';
        for chr in text.chars() {
            if chr == '\n' && chr_prev == '\r' {
                // skip this line break
            } else if chr == '\n' || chr == '\r' {
                self.writeln(out)?;
            } else if chr.is_whitespace() {
                write!(out, " ")?;
                self.column += 1;
            } else {
                self.ensure_indented(out)?;
                for chr_escaped in chr.escape_debug() {
                    write!(out, "{}", chr_escaped)?;
                    self.column += 1;
                }
            }
            chr_prev = chr;
        }
        Ok(())
    }

    /// Writes a new line to the output stream, indenting the cursor by `indent`.
    pub fn writeln<W : io::Write>(
        &mut self,
        out : &mut W,
    ) -> io::Result<()> {
        writeln!(out)?;
        self.column = 0;
        self.do_indent = true;
        Ok(())
    }

    /// Skips the next `n` characters in the output stream.
    pub fn skip<W : io::Write>(
        &mut self,
        out : &mut W,
        n : usize
    ) -> io::Result<()> {
        self.ensure_indented(out)?;
        write!(out, "{}", " ".repeat(n))?;
        self.column += n;
        Ok(())
    }

    fn ensure_indented<W : io::Write>(
        &mut self,
        out : &mut W,
    ) -> io::Result<()> {
        if self.do_indent {
            write!(out, "{}", " ".repeat(self.indent))?;
            self.column += self.indent;
            self.do_indent = false;
        }
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

impl Default for PrettyPrinter {
    fn default() -> Self {
        Self {
            column : 0,
            indent : 0,
            indent_stack : Vec::new(),
            do_indent : true,
        }
    }
}