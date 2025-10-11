use std::io;

use crate::src::{ SourceMap, Message, Location };
use crate::error::{ Diagnostic, IssueManager };
use crate::pretty::{ PrettyPrinter, Colour, Decoration };

/// Renders diagnostic information on a single line:
///
/// ```txt
/// error: message >>> file1.cy:row:col, file2.cy:row:col
/// ```
pub fn write_errors<W : io::Write>(
    printer : &mut PrettyPrinter<W>,
    files : &SourceMap,
    issues : &IssueManager,
) -> io::Result<()> {
    let mut ctx = RendererCtx {
        p : printer, files, message_str : String::new(),
    };
    ctx.write(issues)
}

struct RendererCtx<'p, 'src, W : io::Write> {
    p : &'p mut PrettyPrinter<W>,
    files : &'src SourceMap,
    message_str : String,
}

impl<W : io::Write> RendererCtx<'_, '_, W> {
    fn write(&mut self, issues : &IssueManager) -> io::Result<()> {
        for diag in &issues.errors {
            self.write_diagnostic(diag)?;
        }
        Ok(())
    }

    fn write_diagnostic(&mut self, diag : &Diagnostic) -> io::Result<()> {
        let diag_style = diag.severity.as_colour().decorated(Decoration::Bold);
        self.p.write_style(diag_style)?;
        self.p.write(diag.severity.as_str())?;
        self.p.clear_style()?;
        self.p.write(": ")?;
        // render message
        if let Some(message) = &diag.message {
            self.p.indent_stash();
            self.p.write_style(Decoration::Bold)?;
            self.write_message(message)?;
            self.p.clear_style()?;
            self.p.indent_pop();
            self.p.write(" ")?;
        }
        // render labels
        if !diag.primary_labels.is_empty() {
            self.p.write_style(Colour::BrightCyan)?;
            self.p.write(">>> ")?;
            // render filename
            let mut first = true;
            for label in &diag.primary_labels {
                if !first {
                    self.p.write(", ")?;
                }
                first = false;
                self.write_path(&label.location)?;
            }
            self.p.clear_style()?;
        }
        self.p.write("\n")?;
        Ok(())
    }

    fn write_path(&mut self, location : &Location) -> io::Result<()> {
        self.p.write(&location.show_path(self.files))
    }

    fn write_message(&mut self, message : &Message) -> io::Result<()> {
        self.message_str.clear();
        message.write_to_string(self.files, &mut self.message_str);
        self.p.write(&self.message_str)
    }
}