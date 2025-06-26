use std::{
    env, fs, cmp, fmt, ops,
    ffi::{ OsStr, OsString },
    path::{ Path, PathBuf },
};

/// A simple handle to a file within a package.
pub type FileID = usize;

/// The row and column numbers of a source file.
pub type LineAndColumn = (usize, usize);

/// Represents a span of bytes within a source file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Location {
    /// The span information.
    pub span : Span,
    /// The file this span occurs in.
    pub file_id : FileID,
}

/// Information about a source file.
#[derive(Debug)]
pub struct File {
    /// The directory this file is located in. `None` if the file is not a
    /// physical file.
    pub dir : Option<PathBuf>,
    /// The name of this file, excluding the ext.
    pub name : OsString,
    /// The type of this file.
    pub ext : Option<OsString>,
    /// The source code of the script.
    pub src : String,
    lines : Vec<Span>,
    file_id : FileID,
}

impl File {
    fn calculate_lines(&mut self) {
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

    /// Searches the lines vector for a span that encloses a specific location.
    /// Returns `None` if no lines fit.
    pub fn find_line(&self, pos : usize) -> Option<usize> {
        use cmp::Ordering as ord;
        let comparator = |x : &Span| match x {
            x if x.start > pos => ord::Greater,
            x if x.end < pos => ord::Less,
            _ => ord::Equal,
        };
        self.lines.binary_search_by(comparator).ok().map(|x| x + 1)
    }

    /// Attempts to convert a row number into a file span for this line.
    pub fn find_span(&self, line : usize) -> Option<&Span> {
        self.lines.get(line - 1)
    }

    /// Attempts to convert row and column numbers into a byte index in the
    /// source file.
    pub fn find_index(&self, location : LineAndColumn) -> Option<usize> {
        let line_span = self.find_span(location.0)?;
        Some(line_span.start + location.1 - 1)
    }

    /// Attempts to convert a byte position to a row and column number.
    pub fn find_location(&self, pos : usize) -> Option<LineAndColumn> {
        let line = self.find_line(pos)?;
        let line_span = &self.lines[line - 1];
        Some((line, pos - line_span.start + 1))
    }

    /// Similar to `find_location`, except returns the start and end lines of a
    /// complete span.
    pub fn find_location_span(
        &self,
        span : &Span,
    ) -> Option<(LineAndColumn, LineAndColumn)> {
        let start = self.find_location(span.start)?;
        let end = self.find_location(span.end)?;
        Some((start, end))
    }

    /// Creates a new location from a given span, in the current source file.
    pub fn make_location(&self, span : &Span) -> Location {
        Location {
            span : span.clone(),
            file_id : self.file_id,
        }
    }
}

impl fmt::Display for File {
    fn fmt(&self, out : &mut fmt::Formatter) -> fmt::Result {
        let mut pathbuf = if let Some(path) = &self.dir {
            path.to_path_buf()
        } else {
            PathBuf::new()
        };
        pathbuf.push(&self.name);
        self.ext.as_ref().map(|x| pathbuf.set_extension(x));
        write!(out, "{}", pathbuf.display())
    }
}

/// Stores information about a package.
#[derive(Debug)]
pub struct Package {
    /// A list of source files associated with package.
    pub files : Vec<File>,
    /// The name of this package.
    pub name : OsString,
}

impl Package {
    /// Creates a new package with the given name.
    pub fn new(name : OsString) -> Self {
        Self { files : Vec::new(), name }
    }

    /// Inserts a new virtual file into this package and returns its handle.
    pub fn load_str(
        &mut self,
        mut path : PathBuf,
        src : String,
    ) -> FileID {
        let name = path.file_stem().map(OsString::from).unwrap_or("main".into());
        let ext = path.extension().map(OsString::from);
        let dir = if path.pop() { Some(path) } else { None };
        let file_id = self.files.len();
        let mut file = File {
            dir, name, ext, src,
            lines : Vec::new(),
            file_id,
        };
        file.calculate_lines();
        self.files.push(file);
        file_id
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

/// Represents a span of bytes within a translation unit.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
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
}

impl fmt::Display for Span {
    fn fmt(&self, out : &mut fmt::Formatter) -> fmt::Result {
        write!(out, "[{}..{}]", self.start, self.end)
    }
}