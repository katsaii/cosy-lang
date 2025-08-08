use std::{ env, fs, cmp, fmt, ops, mem };
use std::path::{ Path, PathBuf };
use path_clean::PathClean;
use pathdiff::diff_paths;
use crate::error;

/// A simple handle to a file managed by the compiler.
pub type FileID = usize;

/// The row and column numbers of a source file.
pub type LineAndColumn = (usize, usize);

/// Represents a span of bytes within a source file.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Location {
    /// The span information.
    pub span : Span,
    /// The file this span occurs in.
    pub file_id : FileID,
}

impl Location {
    /// Returns the filename a source location points to in the format
    /// `dirname/filename.ext:line:column`.
    pub fn show_path(&self, files : &FileManager) -> String {
        let file = files.get_file(self.file_id);
        let file_display = file.path.display();
        let (line, column) = file.find_location(self.span.start);
        format!("{}:{}:{}", file_display, line, column)
    }
}

/// Information about a source file.
pub struct File {
    /// The path of this file, including the directory, file name, and extension.
    pub path : PathBuf,
    src : String,
    lines : Vec<Span>,
    file_id : FileID,
}

impl File {
    fn new(
        path : PathBuf,
        src : String,
        file_id : FileID,
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
    pub fn make_location(&self, span : &Span) -> Location {
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
    pub fn load_str(&mut self, path : PathBuf, src : String) -> FileID {
        //let name = path.file_stem().map(OsString::from).unwrap_or("main".into());
        //let ext = path.extension().map(OsString::from);
        //let dir = if path.pop() { Some(path) } else { None };
        let file_id = self.files.len();
        let file = File::new(path, src, file_id);
        self.files.push(file);
        file_id
    }

    /// Opens a physical file and returns its handle.
    pub fn load(&mut self, path : PathBuf) -> error::Result<FileID> {
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

    /// Helper function for getting file information. Since `FileID`s are
    /// only ever created by the compiler, accessing an invalid file will
    /// panic.
    pub fn get_file_mut(&mut self, file : FileID) -> &mut File {
        &mut self.files[file]
    }

    /// Helper function for getting file information. Since `FileID`s are
    /// only ever created by the compiler, accessing an invalid file will
    /// panic.
    pub fn get_file(&self, file : FileID) -> &File {
        &self.files[file]
    }
}

/// Represents a span of bytes within a file.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span {
    /// The starting byte of the span (inclusive).
    pub start : usize,
    /// The ending byte of the span (exclusive).
    pub end : usize,
}

impl Span {
    /// Constructs a new span from this range.
    pub fn new(range : ops::Range<usize>) -> Self {
        Self { start : range.start, end : range.end }
    }

    /// Returns whether the starting byte of the span is greater than or equal
    /// to the ending byte.
    pub fn is_empty(&self) -> bool {
        self.start >= self.end
    }

    /// Returns the byte length of this span.
    ///
    /// If the span is empty, then the length returned is always zero.
    pub fn len(&self) -> usize {
        if self.is_empty() { 0 } else { self.end - self.start }
    }

    /// Joins two spans together using the largest range between them.
    pub fn join(&self, other : &Self) -> Self {
        let start = cmp::min(self.start, other.start);
        let end = cmp::max(self.end, other.end);
        Self::new(start..end)
    }

    /// Joins two spans together using the smallest range between them.
    pub fn diff(&self, other : &Self) -> Self {
        let end = cmp::max(self.start, other.start);
        let start = cmp::min(self.end, other.end);
        Self::new(start..end)
    }

    /// Uses this span to slice a string intro a substring.
    pub fn slice<'a>(&self, src : &'a str) -> &'a str {
        &src[self.start..self.end]
    }

    /// Shrinks this span by `lpad` bytes from the left, and `rpad` bytes from
    /// the right.
    pub fn shrink(&self, lpad : usize, rpad : usize) -> Span {
        let start = self.start + lpad;
        let end = self.end - rpad;
        Self::new(start..end)
    }
}

impl fmt::Display for Span {
    fn fmt(&self, out : &mut fmt::Formatter) -> fmt::Result {
        write!(out, "[{}..{}]", self.start, self.end)
    }
}

/// An owned section of source code, such as a string literal after resolving
/// escape codes.
pub type Symbol = String;