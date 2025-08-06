use std::io;
use crate::{ source::FileManager, error::Diagnostic };
use crate::reporting::{ Renderer, PrettyPrinter, Colour, Style };

/// Renders diagnostic information on a single line:
///
/// ```txt
/// error: message >>> file1.cy:row:col, file2.cy:row:col
/// ```
///
/// Uses coloured text if the output stream supports it. Otherwise, the
/// output will just be monochrome.
pub struct LogRenderer(pub PrettyPrinter);

impl Renderer for LogRenderer {
    fn render_diagnostic<W : io::Write>(
        &mut self,
        out : &mut W,
        diag : &Diagnostic,
        files : &FileManager,
    ) -> io::Result<()> {
        self.0.write_style_fg(out, diag.severity.as_colour())?;
        self.0.write_style(out, Style::Bold)?;
        self.0.write(out, diag.severity.as_str())?;
        self.0.clear_style(out)?;
        self.0.write(out, ": ")?;
        // render messages
        if !diag.messages.is_empty() {
            self.0.indent_stash();
            let mut first = true;
            for message in &diag.messages {
                if !first {
                    self.0.writeln(out)?;
                }
                first = false;
                self.0.write_style(out, Style::Bold)?;
                self.0.write(out, &message.show(files))?;
                self.0.clear_style(out)?;
            }
            self.0.indent_pop();
            self.0.write(out, " ")?;
        }
        // render labels
        if !diag.primary_labels.is_empty() {
            self.0.write_style_fg(out, Colour::BrightCyan)?;
            self.0.write(out, ">>> ")?;
            self.0.clear_style(out)?;
            // render filename
            let mut first = true;
            for label in &diag.primary_labels {
                if !first {
                    self.0.write(out, ", ")?;
                }
                first = false;
                self.0.write(out, &label.location.show_path(files))?;
            }
        }
        self.0.writeln(out)?;
        Ok(())
    }
}