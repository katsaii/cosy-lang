use std::{ io, cmp };
use crate::source::FileManager;
use crate::error::{ Diagnostic, Label, Note, Message, Severity, IssueStats };
use crate::reporting::{ Renderer, PrettyPrinter, Colour, Style };

/// Renders diagnostic information in a pretty format.
///
/// Uses coloured text if the output stream supports it. Otherwise, the
/// output will just be monochrome.
pub struct CliRenderer(pub PrettyPrinter);

impl Renderer for CliRenderer {
    fn render_diagnostic<W : io::Write>(
        &mut self,
        out : &mut W,
        diag : &Diagnostic,
        files : &FileManager,
    ) -> io::Result<()> {
        let colour = diag.severity.as_colour();
        self.0.write_style_fg(out, colour)?;
        self.0.write_style(out, Style::Bold)?;
        self.0.write(out, diag.severity.as_str())?;
        self.0.clear_style(out)?;
        self.0.write(out, ": ")?;
        // render messages
        self.0.write_style(out, Style::Bold)?;
        if !diag.messages.is_empty() {
            self.render_messages(out, &diag.messages, files)?;
        } else {
            self.0.write(out, "no message")?;
        }
        self.0.clear_style(out)?;
        self.0.writeln(out)?;
        // render labels
        for label in &diag.primary_labels {
            self.render_label(out, label, files, colour, '^')?;
        }
        for label in &diag.secondary_labels {
            self.render_label(out, label, files, Colour::BrightBlue, '-')?;
        }
        // render notes
        for note in &diag.notes {
            self.render_note(out, note, files)?;
        }
        self.0.writeln(out)?;
        Ok(())
    }

    fn render_stats<W : io::Write>(
        &mut self,
        out : &mut W,
        stats : &IssueStats,
    ) -> io::Result<()> { 
        let colour = stats.max_severity.as_colour();
        self.0.write_style_fg(out, colour)?;
        self.0.write_style(out, Style::Bold)?;
        self.0.write(out, stats.max_severity.as_str())?;
        self.0.clear_style(out)?;
        self.0.write(out, ": ")?;
        self.0.write(out, &format!("displayed {} message(s)", stats.total()))?;
        self.0.writeln(out)?;
        Ok(())
    }
}

impl CliRenderer {
    fn render_label<W : io::Write>(
        &mut self,
        out : &mut W,
        label : &Label,
        files : &FileManager,
        highlight : Colour,
        highlight_char : char,
    ) -> io::Result<()> {
        let file = files.get_file(label.location.file_id);
        let (start, end) = file.find_location_span(&label.location.span);
        let start_line_n = &start.0.to_string();
        let end_line_n = &end.0.to_string();
        let margin = end_line_n.len();
        // render filename
        self.0.skip(out, margin)?;
        self.0.write_style_fg(out, Colour::BrightCyan)?;
        self.0.write(out, ">>> ")?;
        self.0.write(out, &label.location.show_path(files))?;
        self.0.clear_style(out)?;
        self.0.writeln(out)?;
        // render span
        self.render_margin(out, margin, &start_line_n)?;
        let start_line = file.find_line_span(start.0).unwrap();
        self.0.skip(out, 2)?; // leave 2 spaces for multi-line spans
        self.0.write(out, start_line.slice(file.get_src()))?;
        self.0.writeln(out)?;
        if start.0 == end.0 {
            // render single-line span
            let offset = safe_sub(start.1, 1);
            let length = cmp::max(1, safe_sub(end.1, start.1));
            self.render_margin_end(out, margin)?;
            self.0.skip(out, 2 + offset)?;
            self.0.write_style_fg(out, highlight)?;
            self.0.write_style(out, Style::Bold)?;
            self.0.repeat(out, length, highlight_char)?;
        } else {
            let end_line = file.find_line_span(end.0).unwrap();
            // render multi-line span
            let offset_start = start.1;
            let offset_end = safe_sub(end.1, 1);
            self.render_margin(out, margin, ":")?; // start underline
            self.0.write_style_fg(out, highlight)?;
            self.0.write_style(out, Style::Bold)?;
            self.0.skip(out, 1)?;
            self.0.repeat(out, offset_start, '_')?;
            self.0.repeat(out, 1, highlight_char)?;
            self.0.writeln(out)?;
            self.render_margin(out, margin, &end_line_n)?; // end
            self.0.write_style_fg(out, highlight)?;
            self.0.write_style(out, Style::Bold)?;
            self.0.write(out, "| ")?;
            self.0.clear_style(out)?;
            self.0.write(out, end_line.slice(file.get_src()))?;
            self.0.writeln(out)?;
            self.render_margin_end(out, margin)?; // end underline
            self.0.write_style_fg(out, highlight)?;
            self.0.write_style(out, Style::Bold)?;
            self.0.write(out, "|")?;
            self.0.repeat(out, offset_end, '_')?;
            self.0.repeat(out, 1, highlight_char)?;
        }
        // write captions
        self.0.write(out, " ")?;
        self.render_messages(out, &label.captions, files)?;
        self.0.clear_style(out)?;
        self.0.writeln(out)?;
        Ok(())
    }

    fn render_margin<W : io::Write>(
        &mut self,
        out : &mut W,
        margin : usize,
        text : &str,
    ) -> io::Result<()> {
        self.0.skip(out, margin - text.len())?;
        self.0.write_style_fg(out, Colour::BrightCyan)?;
        self.0.write(out, text)?;
        self.0.write(out, " | ")?;
        self.0.clear_style(out)?;
        Ok(())
    }

    fn render_margin_end<W : io::Write>(
        &mut self,
        out : &mut W,
        margin : usize,
    ) -> io::Result<()> {
        self.0.skip(out, margin)?;
        self.0.write_style_fg(out, Colour::BrightCyan)?;
        self.0.write(out, " ' ")?;
        self.0.clear_style(out)?;
        Ok(())
    }

    fn render_messages<W : io::Write>(
        &mut self,
        out : &mut W,
        messages : &[Message],
        files : &FileManager,
    ) -> io::Result<()> {
        self.0.indent_stash();
        let mut first = true;
        for message in messages {
            if !first {
                self.0.writeln(out)?;
            }
            first = false;
            self.0.write(out, &message.show(files))?;
        }
        self.0.indent_pop();
        Ok(())
    }

    fn render_note<W : io::Write>(
        &mut self,
        out : &mut W,
        note : &Note,
        files : &FileManager,
    ) -> io::Result<()> {
        self.0.write_style_fg(out, Colour::BrightGreen)?;
        self.0.write_style(out, Style::Bold)?;
        self.0.write(out, "note")?;
        self.0.clear_style(out)?;
        self.0.write(out, ": ")?;
        // render messages
        self.render_messages(out, &note.captions, files)?;
        self.0.writeln(out)?;
        Ok(())
    }
}

fn safe_sub(lhs : usize, rhs : usize) -> usize {
    if lhs < rhs { 0 } else { lhs - rhs }
}