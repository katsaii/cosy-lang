use std::{ cmp, fmt, ops };
use bincode::{ Encode, Decode };

use crate::vfs::ManifestFiles;

pub use crate::vfs::FileId;

/// The row and column numbers of a source file.
pub type LineAndColumn = (usize, usize);

/// Points to a file location within the current package/translation unit.
#[derive(Clone, Copy, PartialEq, Eq, Encode, Decode)]
pub struct Location {
    pub span : Span,
    pub file_id : FileId,
}

impl fmt::Debug for Location {
    fn fmt(&self, out : &mut fmt::Formatter) -> fmt::Result {
        write!(out, "<{:?} file {}>", self.span, self.file_id)
    }
}

impl Location {
    /// Returns the filename a source location points to in the format
    /// `dirname/filename.ext:line:column`.
    pub fn show_path(&self, files : &ManifestFiles) -> String {
        let file_meta = files.get_meta(self.file_id).unwrap();
        let file_display = file_meta.path.display();
        let (line, column) = file_meta.find_location(self.span.start);
        format!("{}:{}:{}", file_display, line, column)
    }
}

/// Pairs a value with its location in the source code.
#[derive(Debug, Encode, Decode)]
pub struct Located<T> {
    pub value : T,
    pub loc : Location,
}

/// Represents a span of bytes within a file.
#[derive(Clone, Copy, PartialEq, Eq, Encode, Decode)]
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

    pub(crate) fn find_line(lines : &[Span], pos : usize) -> usize {
        use cmp::Ordering as ord;
        let comparator = |x : &Span| match x {
            x if x.start > pos => ord::Greater,
            x if x.end < pos => ord::Less,
            _ => ord::Equal,
        };
        match lines.binary_search_by(comparator) {
            Ok(x) => x + 1,
            Err(x) => if x < 1 { 1 } else { x }
        }
    }

    pub(crate) fn find_location(lines : &[Span], pos : usize) -> LineAndColumn {
        let line = Span::find_line(lines, pos);
        let line_span = &lines[line - 1];
        (line, pos - line_span.start + 1)
    }
}

impl fmt::Debug for Span {
    fn fmt(&self, out : &mut fmt::Formatter) -> fmt::Result {
        write!(out, "[{}..{}]", self.start, self.end)
    }
}

/// Pairs a value with its location in a file.
#[derive(Debug, Encode, Decode)]
pub struct Spanned<T> {
    pub value : T,
    pub span : Span,
}

impl<T> Spanned<T> {
    pub(crate) fn into_located(self, file_id : FileId) -> Located<T> {
        Located {
            value : self.value,
            loc : Location { span : self.span, file_id }
        }
    }
}