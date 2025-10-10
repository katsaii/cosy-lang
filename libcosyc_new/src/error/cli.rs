use std::{ io, cmp };

use crate::src::{ SourceMap, Message, Location, GetFileResult };
use crate::error::{ Diagnostic, IssueManager, Label, Note };
use crate::pretty::{ PrettyPrinter, Colour, Decoration, Style };

fn safe_sub(lhs : usize, rhs : usize) -> usize {
    if lhs < rhs { 0 } else { lhs - rhs }
}

/// Renders diagnostic information in a pretty format.
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
        let stats = issues.error_stats();
        let stats_total = stats.total();
        if stats_total > 0 {
            let stats_style = stats.max_severity
                .as_colour()
                .decorated(Decoration::Bold);
            self.p.write_style(out, stats_style)?;
            self.p.write(out, stats.max_severity.as_str())?;
            self.p.write_style(out, Style::default())?;
            self.p.write(out,
                &format!(": displayed {} message(s)\n", stats_total)
            )?;
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
        }
        // render labels
        for label in &diag.primary_labels {
            self.write_label(out, label, diag_style, "^")?;
        }
        for label in &diag.secondary_labels {
            let snd_style = Colour::BrightBlue.decorated(Decoration::Bold);
            self.write_label(out, label, snd_style, "-")?;
        }
        // render notes
        for note in &diag.notes {
            self.write_note(out, note)?;
        }
        self.p.write(out, "\n\n")?;
        Ok(())
    }

    fn write_label<W : io::Write>(
        &mut self,
        out : &mut W,
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
        self.p.write(out, "\n")?;
        self.p.skip(out, margin)?;
        self.p.write_style(out, Colour::BrightCyan)?;
        self.p.write(out, ">>> ")?;
        self.write_path(out, &label.location)?;
        self.p.write_style(out, Style::default())?;
        self.p.write(out, "\n")?;
        // render span
        self.write_label_margin(out, margin, &start_line_n)?;
        let start_line = file.find_line_span(start.0).unwrap();
        self.p.skip(out, 2)?; // leave 2 spaces for multi-line spans
        self.p.write(out, start_line.slice(file_src))?;
        self.p.write(out, "\n")?;
        if start.0 == end.0 {
            // render single-line span
            let offset = safe_sub(start.1, 1);
            let length = cmp::max(1, safe_sub(end.1, start.1));
            self.write_label_margin_end(out, margin)?;
            self.p.skip(out, 2 + offset)?;
            self.p.write_style(out, highlight)?;
            self.p.repeat(out, length, highlight_char)?;
        } else {
            let end_line = file.find_line_span(end.0).unwrap();
            // render multi-line span
            let offset_start = start.1;
            let offset_end = safe_sub(end.1, 1);
            self.write_label_margin(out, margin, ":")?; // start underline
            self.p.write_style(out, highlight)?;
            self.p.skip(out, 1)?;
            self.p.repeat(out, offset_start, "_")?;
            self.p.repeat(out, 1, highlight_char)?;
            self.p.write(out, "\n")?;
            self.write_label_margin(out, margin, &end_line_n)?; // end
            self.p.write_style(out, highlight)?;
            self.p.write(out, "| ")?;
            self.p.write_style(out, Style::default())?;
            self.p.write(out, end_line.slice(file_src))?;
            self.p.write(out, "\n")?;
            self.write_label_margin_end(out, margin)?; // end underline
            self.p.write_style(out, highlight)?;
            self.p.write(out, "|")?;
            self.p.repeat(out, offset_end, "_")?;
            self.p.repeat(out, 1, highlight_char)?;
        }
        // write caption
        if let Some(caption) = &label.caption {
            self.p.write(out, " ")?;
            self.write_message(out, caption)?;
        }
        self.p.write_style(out, Style::default())?;
        Ok(())
    }

    fn write_label_margin<W : io::Write>(
        &mut self,
        out : &mut W,
        margin : usize,
        text : &str,
    ) -> io::Result<()> {
        self.p.skip(out, margin - text.len())?;
        self.p.write_style(out, Colour::BrightCyan)?;
        self.p.write(out, text)?;
        self.p.write(out, " | ")?;
        self.p.write_style(out, Style::default())?;
        Ok(())
    }

    fn write_label_margin_end<W : io::Write>(
        &mut self,
        out : &mut W,
        margin : usize,
    ) -> io::Result<()> {
        self.p.skip(out, margin)?;
        self.p.write_style(out, Colour::BrightCyan)?;
        self.p.write(out, " ' ")?;
        self.p.write_style(out, Style::default())?;
        Ok(())
    }

    fn write_note<W : io::Write>(
        &mut self,
        out : &mut W,
        note : &Note,
    ) -> io::Result<()> {
        let note_style = Colour::BrightGreen.decorated(Decoration::Bold);
        self.p.write_style(out, note_style)?;
        self.p.write(out, "\nnote")?;
        self.p.write_style(out, Style::default())?;
        self.p.write(out, ": ")?;
        // render caption
        self.write_message(out, &note.caption)?;
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