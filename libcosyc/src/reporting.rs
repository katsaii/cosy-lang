pub mod log;
pub mod cli;

use std::io;
use crate::{ Session, source::FileManager };
use crate::error::{ Diagnostic, IssueStats };

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
        for diag in &issues.errors {
            self.render_diagnostic(out, diag, files)?;
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
        diag : &Diagnostic,
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
    use_colour : bool,
}

impl PrettyPrinter {
    pub fn new(use_colour : bool) -> Self {
        Self { use_colour, ..Default::default() }
    }

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
            } else if matches!(chr, '\n' | '\r') {
                self.writeln(out)?;
            } else if matches!(chr, '\'' | '"' | '\\') {
                write!(out, "{}", chr)?;
                self.column += 1;
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

    /// Sets the foreground colour of the text that follows. Should have no
    /// effect on unsupported terminals.
    pub fn write_style_fg<W : io::Write>(
        &mut self,
        out : &mut W,
        fg : Colour,
    ) -> io::Result<()> { self.write_ansi(out, fg as usize) }

    /// Sets the background colour of the text that follows. Should have no
    /// effect on unsupported terminals.
    pub fn write_style_bg<W : io::Write>(
        &mut self,
        out : &mut W,
        bg : Colour,
    ) -> io::Result<()> { self.write_ansi(out, bg as usize) }

    /// Sets the style the text that follows. Should have no effect on
    /// unsupported terminals.
    pub fn write_style<W : io::Write>(
        &mut self,
        out : &mut W,
        style : Style,
    ) -> io::Result<()> { self.write_ansi(out, style as usize) }

    /// Clears the current ANSI terminal style.
    pub fn clear_style<W : io::Write>(
        &mut self,
        out : &mut W,
    ) -> io::Result<()> { self.write_ansi(out, 0) }

    fn write_ansi<W : io::Write>(
        &mut self,
        out : &mut W,
        val : usize,
    ) -> io::Result<()> {
        if !self.use_colour {
            return Ok(());
        }
        write!(out, "\x1B[{}m", val)
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
    ) -> io::Result<()> { self.repeat(out, n, ' ') }

    /// Writes a character `n`-many times to the output stream.
    pub fn repeat<W : io::Write>(
        &mut self,
        out : &mut W,
        n : usize,
        chr : char,
    ) -> io::Result<()> {
        assert!(!matches!(chr, '\n' | '\r'), "cannot repeat newline chars");
        self.ensure_indented(out)?;
        for _ in 0..n {
            write!(out, "{}", chr)?;
        }
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
            use_colour : false,
        }
    }
}

/// ANSI Terminal colours.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Colour {
    Black = 30,
    Red = 31,
    Green = 32,
    Yellow = 33,
    Blue = 34,
    Magenta = 35,
    Cyan = 36,
    Grey = 37,
    BrightBlack = 90,
    BrightRed = 91,
    BrightGreen = 92,
    BrightYellow = 93,
    BrightBlue = 94,
    BrightMagenta = 95,
    BrightCyan = 96,
    BrightGrey = 97,
}

/// ANSI Terminal styles.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Style {
    Bold = 1,
    Dimmed = 2,
    Italic = 3,
    Underline = 4,
    Blink = 5,
    Reversed = 7,
    Hidden = 8,
    Strikethrough = 9
}