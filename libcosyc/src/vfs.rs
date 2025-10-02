use bincode::{ Encode, Decode };
use std::path::PathBuf;
use std::hash::{ DefaultHasher, Hash, Hasher };
use std::{ io, fs };

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

pub enum CacheResult {
    Changed(String),
    Unchanged,
    Deleted,
}

/// Maps from `FileId` to file metadata.
#[derive(Debug, Default, Encode, Decode)]
pub struct Manifest(Vec<Option<FileMeta>>);

#[derive(Debug, Encode, Decode)]
struct FileMeta {
    path : PathBuf,
    hash : u64,
    size : usize,
}

impl Manifest {
    /// Gets a reference to the `FileMeta` for this `FileId`, or `None` if
    /// one doesn't exist.
    pub fn get(&self, file_id : FileId) -> Option<&FileMeta> {
        self.0.get(file_id).and_then(|x| x.as_ref())
    }

    /// Opens a file at the given path, registers it to the file manifest, and
    /// then returns its ID along with the file contents.
    pub fn add(&mut self, path : PathBuf) -> io::Result<(FileId, String)> {
        let file_src = fs::read_to_string(&path)?;
        let (hash, size) = hash_str(&file_src);
        let file_meta = Some(FileMeta { path, hash, size });
        let file_id;
        if let Some(pos) = self.0.iter().position(|x| x.is_none()) {
            file_id = pos;
            self.0[pos] = file_meta;
        } else {
            file_id = self.0.len();
            self.0.push(file_meta);
        }
        Ok((file_id, file_src))
    }

    /// Checks whether the contents of a file have changed.
    pub fn check_changed(
        &mut self,
        file_id : FileId
    ) -> io::Result<CacheResult> {
        Ok(CacheResult::Deleted)
    }

    /// Removes a file with the given ID from the manifest. Returns `true` if
    /// the file was removed, and `false` if a file with the given ID doesn't
    /// exist.
    pub fn remove(&mut self, file_id : FileId) -> bool {
        if file_id < self.0.len() {
            self.0[file_id] = None;
            true
        } else {
            false
        }
    }
}