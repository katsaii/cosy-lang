use crate::{
    source::FileManager, error::Diagnostic,
    reporting::{ Renderer, PrettyPrinter, Colour, Style },
};
use std::io;

/// Renders diagnostic information on a single line:
///
/// ```txt
/// error: * message (in file1.cy:row:col, file2.cy:row:col)
/// ```
///
/// Uses coloured text if the output stream supports it. Otherwise, the
/// output will just be monochrome.
pub struct LogRenderer(PrettyPrinter);

impl LogRenderer {
    pub fn new(use_colour : bool) -> Self {
        Self(PrettyPrinter::new(use_colour))
    }
}

impl Renderer for LogRenderer {
    fn render_diagnostic<W : io::Write>(
        &mut self,
        out : &mut W,
        error : &Diagnostic,
        files : &FileManager,
    ) -> io::Result<()> {
        self.0.write_style_fg(out, error.severity.as_colour())?;
        self.0.write_style(out, Style::Bold)?;
        self.0.write(out, error.severity.as_str())?;
        self.0.clear_style(out)?;
        self.0.write(out, ": ")?;
        // render messages
        if !error.messages.is_empty() {
            self.0.indent_stash();
            let mut first = true;
            for message in &error.messages {
                if !first {
                    self.0.writeln(out)?;
                }
                first = false;
                self.0.write_style_fg(out, Colour::BrightCyan)?;
                self.0.write(out, "* ")?;
                self.0.clear_style(out)?;
                self.0.indent_stash();
                self.0.write(out, &message.show(files))?;
                self.0.indent_pop();
            }
            self.0.indent_pop();
            self.0.write(out, " ")?;
        }
        // render labels
        if !error.primary_labels.is_empty() {
            self.0.write(out, "(in ")?;
            let mut first = true;
            for label in &error.primary_labels {
                if !first {
                    self.0.write(out, ", ")?;
                }
                first = false;
                self.0.write(out, &label.location.show_path(files))?;
            }
            self.0.write(out, ")")?;
        }
        self.0.writeln(out)?;
        Ok(())
    }
}