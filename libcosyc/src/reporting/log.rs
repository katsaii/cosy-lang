use crate::{ source::FileManager, reporting::Renderer, error::Diagnostic };
use std::io;

/// Renders diagnostic information on a single line:
///
/// ```txt
/// error: message (in file1.cy:row:col, file2.cy:row:col)
/// ```
///
/// Uses coloured text if the output stream supports it. Otherwise, the
/// output will just be monochrome.
pub struct LogRenderer;

impl Renderer for LogRenderer {
    fn render_diagnostic<W : io::Write>(
        &mut self,
        writer : &mut W,
        error : &Diagnostic,
        files : &FileManager,
    ) -> io::Result<()> {
        write!(writer, "hallo")?;
        Ok(())
    }
}