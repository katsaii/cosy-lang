use std::{ io, cmp };

use crate::src::{ SourceMap, Message, Location, GetFileResult };
use crate::error::{ Diagnostic, IssueManager, Label, Note };
use crate::pretty::{ PrettyPrinter, Colour, Decoration, Style };

fn safe_sub(lhs : usize, rhs : usize) -> usize {
    if lhs < rhs { 0 } else { lhs - rhs }
}

/// Renders diagnostic information in a pretty format.
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
        let stats = issues.error_stats();
        let stats_total = stats.total();
        if stats_total > 0 {
            let stats_style = stats.max_severity
                .as_colour()
                .decorated(Decoration::Bold);
            self.p.write_style(stats_style)?;
            self.p.write(stats.max_severity.as_str())?;
            self.p.write_style(Style::default())?;
            self.p.write(&format!(": displayed {} message(s)\n", stats_total))?;
        }
        Ok(())
    }

    fn write_diagnostic(&mut self, diag : &Diagnostic) -> io::Result<()> {
        let diag_style = diag.severity.as_colour().decorated(Decoration::Bold);
        self.p.write_style(diag_style)?;
        self.p.write(diag.severity.as_str())?;
        self.p.write_style(Style::default())?;
        self.p.write(": ")?;
        // render message
        if let Some(message) = &diag.message {
            self.p.indent_stash();
            self.p.write_style(Decoration::Bold)?;
            self.write_message(message)?;
            self.p.write_style(Style::default())?;
            self.p.indent_pop();
        }
        // render labels
        for label in &diag.primary_labels {
            self.write_label(label, diag_style, "^")?;
        }
        for label in &diag.secondary_labels {
            let snd_style = Colour::BrightBlue.decorated(Decoration::Bold);
            self.write_label(label, snd_style, "-")?;
        }
        // render notes
        for note in &diag.notes {
            self.write_note(note)?;
        }
        self.p.write("\n\n")?;
        Ok(())
    }

    fn write_label(
        &mut self,
        label : &Label,
        highlight : Style,
        highlight_char : &'static str,
    ) -> io::Result<()> {
        assert!(highlight_char.len() == 1);
        let file = match self.files.get_existing_file(label.location.file_id) {
            GetFileResult::Ok(_, file) => file,
            GetFileResult::ErrNotInManifest => return Ok(()),
            GetFileResult::ErrIo(err) => return Err(err),
        };
        let file_src = &file.src;
        let (start, end) = file.find_line_and_col_span(&label.location.span);
        let start_line_n = &start.0.to_string();
        let end_line_n = &end.0.to_string();
        let margin = end_line_n.len();
        // render filename
        self.p.write("\n")?;
        self.p.skip(margin)?;
        self.p.write_style(Colour::BrightCyan)?;
        self.p.write(">>> ")?;
        self.write_path(&label.location)?;
        self.p.write_style(Style::default())?;
        self.p.write("\n")?;
        // render span
        self.write_label_margin(margin, &start_line_n)?;
        let start_line = file.find_line_span(start.0).unwrap();
        self.p.skip(2)?; // leave 2 spaces for multi-line spans
        self.p.write(start_line.slice(file_src))?;
        self.p.write("\n")?;
        if start.0 == end.0 {
            // render single-line span
            let offset = safe_sub(start.1, 1);
            let length = cmp::max(1, safe_sub(end.1, start.1));
            self.write_label_margin_end(margin)?;
            self.p.skip(2 + offset)?;
            self.p.write_style(highlight)?;
            self.p.repeat(length, highlight_char)?;
        } else {
            let end_line = file.find_line_span(end.0).unwrap();
            // render multi-line span
            let offset_start = start.1;
            let offset_end = safe_sub(end.1, 1);
            self.write_label_margin(margin, ":")?; // start underline
            self.p.write_style(highlight)?;
            self.p.skip(1)?;
            self.p.repeat(offset_start, "_")?;
            self.p.repeat(1, highlight_char)?;
            self.p.write("\n")?;
            self.write_label_margin(margin, &end_line_n)?; // end
            self.p.write_style(highlight)?;
            self.p.write("| ")?;
            self.p.write_style(Style::default())?;
            self.p.write(end_line.slice(file_src))?;
            self.p.write("\n")?;
            self.write_label_margin_end(margin)?; // end underline
            self.p.write_style(highlight)?;
            self.p.write("|")?;
            self.p.repeat(offset_end, "_")?;
            self.p.repeat(1, highlight_char)?;
        }
        // write caption
        if let Some(caption) = &label.caption {
            self.p.write(" ")?;
            self.write_message(caption)?;
        }
        self.p.write_style(Style::default())?;
        Ok(())
    }

    fn write_label_margin(
        &mut self,
        margin : usize,
        text : &str,
    ) -> io::Result<()> {
        self.p.skip(margin - text.len())?;
        self.p.write_style(Colour::BrightCyan)?;
        self.p.write(text)?;
        self.p.write(" | ")?;
        self.p.write_style(Style::default())?;
        Ok(())
    }

    fn write_label_margin_end(&mut self, margin : usize) -> io::Result<()> {
        self.p.skip(margin)?;
        self.p.write_style(Colour::BrightCyan)?;
        self.p.write(" ' ")?;
        self.p.write_style(Style::default())?;
        Ok(())
    }

    fn write_note(&mut self, note : &Note) -> io::Result<()> {
        let note_style = Colour::BrightGreen.decorated(Decoration::Bold);
        self.p.write_style(note_style)?;
        self.p.write("\nnote")?;
        self.p.write_style(Style::default())?;
        self.p.write(": ")?;
        // render caption
        self.write_message(&note.caption)?;
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