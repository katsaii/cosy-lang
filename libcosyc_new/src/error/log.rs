use std::io;

use crate::src::{ SourceMap, Message, Location };
use crate::error::{ Diagnostic, IssueManager };
use crate::pretty::{ PrettyPrinter, Colour, Decoration, Style };

/// Renders diagnostic information on a single line:
///
/// ```txt
/// error: message >>> file1.cy:row:col, file2.cy:row:col
/// ```
///
/// Uses coloured text if the output stream supports it. Otherwise, the
/// output will just be monochrome.
pub fn write_errors<W : io::Write>(
    printer : &mut PrettyPrinter,
    out : &mut W,
    files : &SourceMap,
    issues : &IssueManager,
) -> io::Result<()> {
    let mut ctx = RendererCtx {
        p : printer, files, message_str : String::new(),
    };
    ctx.write(out, issues)
}

struct RendererCtx<'p, 'src> {
    p : &'p mut PrettyPrinter,
    files : &'src SourceMap,
    message_str : String,
}

impl RendererCtx<'_, '_> {
    fn write<W : io::Write>(
        &mut self,
        out : &mut W,
        issues : &IssueManager,
    ) -> io::Result<()> {
        for diag in &issues.errors {
            self.write_diagnostic(out, diag)?;
        }
        Ok(())
    }

    fn write_diagnostic<W : io::Write>(
        &mut self,
        out : &mut W,
        diag : &Diagnostic,
    ) -> io::Result<()> {
        let diag_style = diag.severity.as_colour().decorated(Decoration::Bold);
        self.p.write_style(out, diag_style)?;
        self.p.write(out, diag.severity.as_str())?;
        self.p.write_style(out, Style::default())?;
        self.p.write(out, ": ")?;
        // render message
        if let Some(message) = &diag.message {
            self.p.indent_stash();
            self.p.write_style(out, Decoration::Bold)?;
            self.write_message(out, message)?;
            self.p.write_style(out, Style::default())?;
            self.p.indent_pop();
            self.p.write(out, " ")?;
        }
        // render labels
        if !diag.primary_labels.is_empty() {
            self.p.write_style(out, Colour::BrightCyan)?;
            self.p.write(out, ">>> ")?;
            // render filename
            let mut first = true;
            for label in &diag.primary_labels {
                if !first {
                    self.p.write(out, ", ")?;
                }
                first = false;
                self.write_path(out, &label.location)?;
            }
            self.p.write_style(out, Style::default())?;
        }
        self.p.write(out, "\n")?;
        Ok(())
    }

    fn write_path<W : io::Write>(
        &mut self,
        out : &mut W,
        location : &Location,
    ) -> io::Result<()> {
        self.p.write(out, &location.show_path(self.files))
    }

    fn write_message<W : io::Write>(
        &mut self,
        out : &mut W,
        message : &Message,
    ) -> io::Result<()> {
        self.message_str.clear();
        message.write_to_string(self.files, &mut self.message_str);
        self.p.write(out, &self.message_str)
    }
}