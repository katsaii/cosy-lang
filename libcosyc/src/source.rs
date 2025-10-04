use std::{ env, fs, cmp, fmt, mem };
use std::path::{ Path, PathBuf };
use path_clean::PathClean;
use pathdiff::diff_paths;
use bincode::{ Encode, Decode };

pub use crate::vfs::{ FileId, Span, Location, LineAndColumn };

/// Pairs a value with its location the source code.
#[derive(Encode, Decode)]
pub struct SourceRef<T> {
    pub value : T,
    pub loc : Location,
}

impl<T : fmt::Debug> fmt::Debug for SourceRef<T> {
    fn fmt(&self, out : &mut fmt::Formatter) -> fmt::Result {
        self.value.fmt(out)
    }
}

/// Information about a source file.
pub struct File {
    /// The path of this file, including the directory, file name, and extension.
    pub path : PathBuf,
    src : String,
    lines : Vec<Span>,
    file_id : FileId,
}

impl File {
    fn new(
        path : PathBuf,
        src : String,
        file_id : FileId,
    ) -> Self {
        let mut file = Self { path, src, lines : vec![], file_id };
        file.refresh_lines();
        file
    }

    fn refresh_lines(&mut self) {
        self.lines.clear();
        let mut chars = self.src.char_indices().peekable();
        let mut start = 0;
        while let Some((end, next)) = chars.next() {
            if !matches!(next, '\r' | '\n') {
                continue;
            }
            if matches!(next, '\r') &&
                    matches!(chars.peek(), Some((_, '\n'))) {
                // ignore CRLF
                chars.next();
            }
            self.lines.push(Span::new(start..end));
            start = if let Some((i, _)) = chars.peek() {
                *i
            } else {
                self.src.len()
            };
        }
        self.lines.push(Span::new(start..self.src.len()));
        self.lines.shrink_to_fit();
    }

    /// Returns the contents of this file.
    pub fn get_src(&self) -> &str { &self.src }

    /// Updates the contents of this file, returning ownership of the old content.
    pub fn set_src(&mut self, src : String) -> String {
        let old_src = mem::replace(&mut self.src, src);
        self.refresh_lines();
        old_src
    }

    /// Searches the lines vector for a span that encloses a specific location.
    pub fn find_line(&self, pos : usize) -> usize {
        use cmp::Ordering as ord;
        let comparator = |x : &Span| match x {
            x if x.start > pos => ord::Greater,
            x if x.end < pos => ord::Less,
            _ => ord::Equal,
        };
        match self.lines.binary_search_by(comparator) {
            Ok(x) => x + 1,
            Err(x) => if x < 1 { 1 } else { x }
        }
    }

    /// Attempts to convert a row number into a file span for this line.
    pub fn find_line_span(&self, line : usize) -> Option<&Span> {
        self.lines.get(line - 1)
    }

    /// Attempts to convert a byte position to a row and column number.
    pub fn find_location(&self, pos : usize) -> LineAndColumn {
        let line = self.find_line(pos);
        let line_span = &self.lines[line - 1];
        (line, pos - line_span.start + 1)
    }

    /// Similar to `find_location`, except returns the start and end lines of a
    /// complete span.
    pub fn find_location_span(
        &self,
        span : &Span,
    ) -> (LineAndColumn, LineAndColumn) {
        let start = self.find_location(span.start);
        let mut end = self.find_location(span.end);
        if end.0 > start.0 && end.1 == 1 {
            // try correct spans that end in the newline character
            if let Some(prev_line) = self.find_line_span(end.0 - 1) {
                end.0 -= 1;
                end.1 = prev_line.len() + 1;
            }
        }
        (start, end)
    }

    /// Creates a new location from a given span, in the current source file.
    pub fn location(&self, span : &Span) -> Location {
        Location {
            span : span.clone(),
            file_id : self.file_id,
        }
    }
}

/// Converts an arbitrary path into an absolute path, using the current working
/// directory as its root if the path is relative.
///
/// Will resolve any symlinks.
///
/// Returns `None` if the path is unchanged.
pub fn resolve_absolute_path(path : &Path) -> Option<PathBuf> {
    if let Some(canon_path) = fs::canonicalize(path).ok() {
        Some(canon_path)
    } else if path.is_absolute() {
        None
    } else if let Some(cwd) = env::current_dir().ok() {
        Some(cwd.join(path).clean())
    } else {
        None
    }
}

/// Converts an absolute path into a path relative to the current working
/// directory.
///
/// Returns `None` if the path is not relative to the working directory.
pub fn resolve_relative_path(path : &Path) -> Option<PathBuf> {
    diff_paths(path, env::current_dir().ok()?)
}

/*
/// Stores information about files managed by the compiler. Avoids having to
/// pass around lots of `Rc<File>` instances around.
///
/// File ids are also a lot smaller and more convenient to clone.
#[derive(Default)]
pub struct FileManager {
    files : Vec<File>,
}

impl FileManager {
    /// "Opens" a new virtual file and returns its handle.
    pub fn load_str(&mut self, path : PathBuf, src : String) -> FileId {
        //let name = path.file_stem().map(OsString::from).unwrap_or("main".into());
        //let ext = path.extension().map(OsString::from);
        //let dir = if path.pop() { Some(path) } else { None };
        let file_id = self.files.len();
        let file = File::new(path, src, file_id);
        self.files.push(file);
        file_id
    }

    /// Opens a physical file and returns its handle.
    pub fn load(&mut self, path : PathBuf) -> error::Result<FileId> {
        let path = resolve_absolute_path(&path).unwrap_or(path);
        let path = resolve_relative_path(&path).unwrap_or(path);
        let src = match fs::read_to_string(&path) {
            Ok(x) => x,
            Err(err) => {
                let diag = error::Diagnostic::error()
                    .message(("failed to load file `{}`: {}", [
                        path.display().to_string().into(),
                        err.into(),
                    ]));
                return Err(diag)
            }
        };
        let file_id = self.load_str(path, src);
        Ok(file_id)
    }

    /// Helper function for getting file information. Since `FileId`s are
    /// only ever created by the compiler, accessing an invalid file will
    /// panic.
    pub fn get_file_mut(&mut self, file : FileId) -> &mut File {
        &mut self.files[file]
    }

    /// Helper function for getting file information. Since `FileId`s are
    /// only ever created by the compiler, accessing an invalid file will
    /// panic.
    pub fn get_file(&self, file : FileId) -> &File {
        &self.files[file]
    }
}
*/

/// An owned section of source code, such as a string literal after resolving
/// escape codes.
pub type Symbol = String;