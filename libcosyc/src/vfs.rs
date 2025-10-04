use bincode::{ Encode, Decode };
use std::path::{ Path, PathBuf };
use std::hash::{ DefaultHasher, Hash, Hasher };
use std::collections::HashMap;
use std::{ io, fs, cmp };

use crate::source::{ Span, Location, LineAndColumn };

/// A simple handle to a file managed by the compiler.
pub type FileId = usize;

fn hash_str(s : &str) -> (u64, usize) {
    let hash = {
        let mut hasher = DefaultHasher::new();
        s.hash(&mut hasher);
        hasher.finish()
    };
    (hash, s.len())
}

fn get_lines(lines : &mut Vec<Span>, src : &str) {
    lines.clear();
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
}

/// Maps from `FileId` to file metadata.
#[derive(Debug, Default, Encode, Decode)]
pub struct Manifest {
    id_2_meta : HashMap<FileId, FileMeta>,
    path_2_id : HashMap<PathBuf, FileId>,
    next_id : usize,
}

impl Manifest {
    /// Returns metadata about the file with the given ID.
    pub fn get(&self, file_id : FileId) -> Option<&FileMeta> {
        self.id_2_meta.get(&file_id)
    }

    /// Loads a file with the given path; returning its ID, its content, and
    /// whether the file has been modified.
    pub fn load(&mut self, path : &Path) -> io::Result<FileData> {
        let file_src = match fs::read_to_string(path) {
            Ok(ok) => ok,
            Err(err) => {
                // delete the cache
                if let Some(file_id) = self.path_2_id.get(path) {
                    self.id_2_meta.remove(file_id);
                    self.path_2_id.remove(path);
                }
                return Err(err);
            }
        };
        let (hash, size) = hash_str(&file_src);
        let (file_id, modified) = if let Some(file_id) = self.path_2_id.get(path) {
            let file_id = *file_id;
            // file already exists, check the cache
            let file_meta = self.id_2_meta.get(&file_id).unwrap();
            (file_id, file_meta.hash != hash || file_meta.size != size)
        } else {
            // file doesn't exist, add it
            let file_id = self.next_id;
            self.next_id += 1;
            let file_meta = FileMeta {
                path : path.to_owned(),
                lines : vec![],
                hash, size,
            };
            self.id_2_meta.insert(file_id, file_meta);
            self.path_2_id.insert(path.to_owned(), file_id);
            (file_id, true)
        };
        if modified {
            let file_meta = self.id_2_meta.get_mut(&file_id).unwrap();
            get_lines(&mut file_meta.lines, &file_src);
        }
        Ok(FileData { id : file_id, src : file_src, modified })
    }

    /// Reads the contents of the files in this manifest and returns them.
    pub fn read_files(&self) -> io::Result<ManifestFiles> {
        let mut files = HashMap::new();
        for (path, file_id) in &self.path_2_id {
            let file = fs::read_to_string(path)?;
            files.insert(*file_id, file);
        }
        Ok(ManifestFiles { manifest : self, files })
    }
}

pub struct ManifestFiles<'m> {
    manifest : &'m Manifest,
    files : HashMap<FileId, String>,
}

impl<'m> ManifestFiles<'m> {
    /// Gets the source code of a file with the given ID.
    pub fn get_src<'a>(&'a self, file_id : FileId) -> Option<&'a str> {
        self.files.get(&file_id).map(|x| x.as_str())
    }

    /// Gets the metadata of a file with the given ID.
    pub fn get_meta(&self, file_id : FileId) -> Option<&'m FileMeta> {
        self.manifest.get(file_id)
    }
}

#[derive(Debug, Encode, Decode)]
pub struct FileMeta {
    pub path : PathBuf,
    pub lines : Vec<Span>,
    pub hash : u64,
    pub size : usize,
}

impl FileMeta {
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
}

#[derive(Debug)]
pub struct FileData {
    pub id : FileId,
    pub src : String,
    pub modified : bool,
}

impl FileData {
    /// Creates a new location from a given span, in the current source file.
    pub fn location(&self, span : &Span) -> Location {
        Location {
            span : span.clone(),
            file_id : self.id,
        }
    }
}