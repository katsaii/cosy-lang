use bincode;

use std::cell::RefCell;
use std::sync::Arc;
//use std::time::SystemTime;
use std::path::{ PathBuf, Path };
use std::hash::{ DefaultHasher, Hash, Hasher };
use std::collections::HashMap;
use std::{ fmt, fs, ops, cmp, io };

/// A simple handle to a file managed by the compiler.
pub type FileId = u64;

/// A hash of the contents of a file. Only really needs to be used to check if
/// a specific file has changed, so the Birthday Paradox isn't going to be a
/// problem. (Hopefully.)
pub type FileHash = u64;

fn str_get_hash(s : &str) -> FileHash {
    let mut hasher = DefaultHasher::new();
    s.hash(&mut hasher);
    hasher.finish()
}

fn str_get_lines(src : &str) -> Vec<Span> {
    let mut lines = Vec::new();
    let mut chars = src.char_indices().peekable();
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
        lines.push(Span::new(start..end));
        start = if let Some((i, _)) = chars.peek() {
            *i
        } else {
            src.len()
        };
    }
    lines.push(Span::new(start..src.len()));
    lines.shrink_to_fit();
    lines
}

/// Maps from virtual file ids to source files.
#[derive(Debug, Default)]
pub struct SourceMap {
    manifest : Manifest,
    files : RefCell<HashMap<FileId, Arc<SourceFile>>>,
}

#[derive(Debug, Default, bincode::Encode, bincode::Decode)]
struct Manifest {
    path_2_id : HashMap<PathBuf, FileId>,
    id_2_file : HashMap<FileId, ManifestFile>,
    next_id : FileId,
}

#[derive(Debug, bincode::Encode, bincode::Decode)]
struct ManifestFile {
    path : PathBuf,
    hash : FileHash,
    // TODO :: implement system time into caching
    //system_time : SystemTime,
}

impl SourceMap {
    /// Creates a new blank source map.
    pub fn new() -> SourceMap { SourceMap::default() }
}

#[derive(Debug)]
pub enum LoadManifestResult {
    Ok(SourceMap),
    ErrIo(io::Error),
    ErrBincode(bincode::error::DecodeError),
}

impl SourceMap {
    /// Attempts to load a source map from a manifest file if one exists at the
    /// given path.
    ///
    /// Returns `None` if the manifest file could not be loaded, or doesn't exist.
    pub fn load_from_path(path : &Path) -> LoadManifestResult {
        let config = bincode::config::standard();
        let mut file = match fs::File::open(path) {
            Ok(ok) => ok,
            Err(err) => return LoadManifestResult::ErrIo(err),
        };
        let manifest : Manifest = match bincode::decode_from_std_read(
            &mut file, config
        ) {
            Ok(ok) => ok,
            Err(err) => return LoadManifestResult::ErrBincode(err),
        };
        LoadManifestResult::Ok(SourceMap { manifest, ..Default::default() })
    }
}

#[derive(Debug)]
pub enum SaveManifestResult {
    Ok,
    ErrIo(io::Error),
    ErrBincode(bincode::error::EncodeError),
}

impl SourceMap {
    /// Attempts to write the manifest of this source map to a file at the
    /// given path.
    ///
    /// Returns `true` if this was successful, and `false` otherwise.
    pub fn save_to_path(&self, path : &Path) -> SaveManifestResult {
        let config = bincode::config::standard();
        let mut file = match fs::File::create(path) {
            Ok(ok) => ok,
            Err(err) => return SaveManifestResult::ErrIo(err),
        };
        if let Err(err) = bincode::encode_into_std_write(
            &self.manifest, &mut file, config
        ) {
            return SaveManifestResult::ErrBincode(err);
        }
        SaveManifestResult::Ok
    }

    fn add_file(&self, file_id : FileId, src : String) -> Arc<SourceFile> {
        let lines = str_get_lines(&src);
        let file = Arc::new(SourceFile { id : file_id, src, lines });
        let mut files = self.files.borrow_mut();
        files.insert(file_id, file.to_owned());
        file
    }
}

#[derive(Debug)]
pub enum GetFileResult<'path> {
    Ok((&'path Path, Arc<SourceFile>)),
    ErrNotInManifest,
    ErrIo(io::Error),
}

impl SourceMap {
    /// Returns a reference to an existing source file with this ID if it
    /// has previously been opened, or attempts to load the file from disk if
    /// it appears in the source map manifest.
    ///
    /// Returns `NotInManifest` if `file_id` is not in the manifest.
    /// Returns `io::Error` if there was an error loading the source file, e.g.
    /// the file has been deleted.
    pub fn get_existing_file<'path>(
        &'path self,
        file_id : FileId,
    ) -> GetFileResult<'path> {
        let path = match self.manifest.id_2_file.get(&file_id) {
            Some(ManifestFile { path, .. }) => path,
            None => return GetFileResult::ErrNotInManifest,
        };
        if let Some(file) = self.files.borrow().get(&file_id) {
            GetFileResult::Ok((path, file.to_owned()))
        } else {
            let src = match fs::read_to_string(path) {
                Ok(ok) => ok,
                Err(err) => return GetFileResult::ErrIo(err),
            };
            GetFileResult::Ok((path, self.add_file(file_id, src)))
        }
    }
}

#[derive(Debug)]
pub enum LoadFileResult {
    Ok(Arc<SourceFile>),
    OkUnchanged(FileId),
    ErrIo(io::Error),
}

impl SourceMap {
    /// Loads a file at the given path, and returns its source content only if
    /// it was new or modified. If the file was not modified, then this
    /// function does nothing.
    pub fn load_file_if_new_or_modified(
        &mut self,
        path : &Path,
    ) -> LoadFileResult {
        let file = if let Some(file_id) = self.manifest.path_2_id.get(path) {
            // file already exists in the cache, check hash
            let cache = self.manifest.id_2_file.get_mut(file_id).unwrap();
            if let Some(file) = self.files.borrow().get(file_id) {
                // if the file was already loaded, use that instead of loading
                // the same file twice, this should be a rare path
                let hash = str_get_hash(&file.src);
                if cache.hash == hash {
                    return LoadFileResult::OkUnchanged(*file_id);
                }
                cache.hash = hash;
                file.to_owned()
            } else {
                let src = match fs::read_to_string(path) {
                    Ok(ok) => ok,
                    Err(err) => return LoadFileResult::ErrIo(err),
                };
                let hash = str_get_hash(&src);
                if cache.hash == hash {
                    return LoadFileResult::OkUnchanged(*file_id);
                }
                cache.hash = hash;
                self.add_file(*file_id, src)
            }
        } else {
            // new file
            let src = match fs::read_to_string(path) {
                Ok(ok) => ok,
                Err(err) => return LoadFileResult::ErrIo(err),
            };
            let file_id = self.manifest.next_id;
            self.manifest.next_id += 1;
            let cache = ManifestFile {
                path : path.to_owned(),
                hash : str_get_hash(&src),
            };
            self.manifest.id_2_file.insert(file_id, cache);
            self.manifest.path_2_id.insert(path.to_owned(), file_id);
            self.add_file(file_id, src)
        };
        LoadFileResult::Ok(file)
    }

    /// Loads a file at the given path, and returns its source content
    /// regardless of whether it was modified.
    pub fn load_file(
        &mut self,
        path : &Path,
    ) -> io::Result<Arc<SourceFile>> {
        let file_id = match self.load_file_if_new_or_modified(path) {
            LoadFileResult::Ok(file) => return Ok(file),
            LoadFileResult::ErrIo(err) => return Err(err),
            LoadFileResult::OkUnchanged(file_id) => file_id,
        };
        match self.get_existing_file(file_id) {
            GetFileResult::Ok((_, file)) => return Ok(file),
            GetFileResult::ErrNotInManifest => unreachable!(),
            GetFileResult::ErrIo(err) => return Err(err),
        }
    }
}

/// Stores information about a currently loaded source file.
#[derive(Debug)]
pub struct SourceFile {
    pub id : FileId,
    /// The complete source code.
    pub src : String,
    /// The lines within the source code.
    pub lines : Vec<Span>,
}

impl SourceFile {
    /// Creates a new location from a given span, in the current source file.
    pub fn location(&self, span : &Span) -> Location {
        Location {
            span : span.clone(),
            file_id : self.id,
        }
    }

    /// Searches the lines vector for a span that encloses a specific location.
    pub fn find_line(&self, pos : usize) -> usize {
        find_line(&self.lines, pos)
    }

    /// Attempts to convert a row number into a file span for this line.
    pub fn find_line_span(&self, line : usize) -> Option<&Span> {
        self.lines.get(line - 1)
    }

    /// Attempts to convert a byte position to a row and column number.
    pub fn find_line_and_col(&self, pos : usize) -> (usize, usize) {
        find_line_and_col(&self.lines, pos)
    }

    /// Similar to `find_location`, except returns the start and end lines of a
    /// complete span.
    pub fn find_line_and_col_span(
        &self,
        span : &Span,
    ) -> ((usize, usize), (usize, usize)) {
        let start = self.find_line_and_col(span.start);
        let mut end = self.find_line_and_col(span.end);
        if end.0 > start.0 && end.1 == 1 {
            // try correct spans that end in the newline character
            if let Some(prev_line) = self.find_line_span(end.0 - 1) {
                end.0 -= 1;
                end.1 = prev_line.len() + 1;
            }
        }
        (start, end)
    }
}

/// Points to a file location within the current package/translation unit.
#[derive(Clone, Copy, PartialEq, Eq, bincode::Encode, bincode::Decode)]
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
    /// `filename.ext:line:column`.
    pub fn show_path(&self, source_map : &SourceMap) -> String {
        match source_map.get_existing_file(self.file_id) {
            GetFileResult::Ok((path, file)) => {
                let (line, column) = file.find_line_and_col(self.span.start);
                format!("{}:{}:{}", path.display(), line, column)
            },
            _ => format!("{:?}", self),
        }
    }

    /// Copies the contents of this location from the source file to a
    /// desination string.
    ///
    /// Returns the number of bytes written to `dest`.
    pub fn write_to_string(
        &self,
        source_map : &SourceMap,
        dest : &mut String
    ) -> usize {
        match source_map.get_existing_file(self.file_id) {
            GetFileResult::Ok((_, file)) => {
                let src_str = self.span.slice(&file.src);
                dest.push_str(src_str);
                src_str.len()
            },
            _ => 0,
        }
    }
}

/// Pairs a value with its location in the source code.
#[derive(Debug, bincode::Encode, bincode::Decode)]
pub struct Located<T> {
    pub value : T,
    pub loc : Location,
}

/// Represents a span of bytes within a file.
#[derive(Clone, Copy, PartialEq, Eq, bincode::Encode, bincode::Decode)]
pub struct Span {
    /// The starting byte of the span (inclusive).
    pub start : usize,
    /// The ending byte of the span (exclusive).
    pub end : usize,
}

impl fmt::Debug for Span {
    fn fmt(&self, out : &mut fmt::Formatter) -> fmt::Result {
        write!(out, "[{}..{}]", self.start, self.end)
    }
}

impl Span {
    /// Constructs a new span from this range.
    pub fn new(range : ops::Range<usize>) -> Span {
        Span { start : range.start, end : range.end }
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
    pub fn join(&self, other : &Span) -> Span {
        let start = cmp::min(self.start, other.start);
        let end = cmp::max(self.end, other.end);
        Span::new(start..end)
    }

    /// Joins two spans together using the smallest range between them.
    pub fn diff(&self, other : &Span) -> Span {
        let end = cmp::max(self.start, other.start);
        let start = cmp::min(self.end, other.end);
        Span::new(start..end)
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
        Span::new(start..end)
    }
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

pub(crate) fn find_line_and_col(
    lines : &[Span],
    pos : usize
) -> (usize, usize) {
    let line = find_line(lines, pos);
    let line_span = &lines[line - 1];
    (line, pos - line_span.start + 1)
}

/// Represents a complete message that may contain source information.
pub struct Message {
    template : &'static str,
    args : Vec<TextFragment>,
}

impl From<&'static str> for Message {
    fn from(template : &'static str) -> Message {
        Message { template, args : Vec::new() }
    }
}

impl<I : IntoIterator<Item=TextFragment>> From<(&'static str, I)> for Message {
    fn from((template, args) : (&'static str, I)) -> Message {
        Message { template, args : args.into_iter().collect() }
    }
}

impl Message {
    /// Copies the contents of this message to a desination string.
    ///
    /// Returns the number of bytes written to `dest`.
    pub fn write_to_string(
        &self,
        source_map : &SourceMap,
        dest : &mut String
    ) -> usize {
        let mut arg_n = 0;
        let mut template_slice = self.template;
        let mut bytes = 0;
        while let Some(end) = template_slice.find("{}") {
            let prefix = &template_slice[..end];
            bytes += prefix.len();
            dest.push_str(prefix);
            template_slice = &template_slice[end + 2..];
            // write arg
            if let Some(arg) = self.args.get(arg_n) {
                bytes += arg.write_to_string(source_map, dest);
            } else {
                bytes += 2;
                dest.push_str("{}");
            }
            arg_n += 1;
        }
        bytes += template_slice.len();
        dest.push_str(template_slice);
        bytes
    }
}

/// Represents a string or piece of source code.
#[derive(PartialEq, Eq)]
pub enum TextFragment {
    Text(String),
    Code(Location),
}

impl From<Location> for TextFragment {
    fn from(location : Location) -> TextFragment {
        TextFragment::Code(location)
    }
}

impl<S : ToString> From<S> for TextFragment {
    fn from(s : S) -> TextFragment {
        TextFragment::Text(s.to_string())
    }
}

impl TextFragment {
    /// Copies the contents of this text fragment to a desination string.
    ///
    /// Returns the number of bytes written to `dest`.
    pub fn write_to_string(
        &self,
        source_map : &SourceMap,
        dest : &mut String
    ) -> usize {
        match self {
            TextFragment::Text(src) => {
                dest.push_str(&src);
                src.len()
            },
            TextFragment::Code(location) => {
                location.write_to_string(source_map, dest)
            },
        }
    }
}