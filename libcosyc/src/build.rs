use std::path::{ Path, PathBuf };
use std::fs;

use crate::src::{ SourceMap, LoadFileResult, GetFileResult };
use crate::error::{ Diagnostic, IssueManager };
use crate::ir::{ ast, hir, casm };

/// Parses a module into its HIR representation, inferring the types of
/// all variables as best it can at this stage.
///
/// If the file already exists and was unmodified, then the cached version of
/// the HIR will be returned if it exists.
///
/// Reports any errors to `issues`.
pub fn build_module(
    files : &mut SourceMap,
    issues : &mut IssueManager,
    cache_dir : &Path,
    module_path : &Path,
) -> Option<hir::Module> {
    let result = match files.load_file_if_new_or_modified(module_path) {
        LoadFileResult::Ok(ok) => Ok(ok),
        LoadFileResult::OkUnchanged(file_id) => {
            // load from cache
            let cache_path = cache_dir.join(format!("{}.bin", file_id));
            if let Ok(mut file) = fs::File::open(cache_path) {
                let config = bincode::config::standard();
                match bincode::decode_from_std_read(&mut file, config) {
                    Ok(ok) => return Some(ok),
                    Err(_) => (),
                };
            }
            match files.get_existing_file(file_id) {
                GetFileResult::Ok((_, file)) => Ok(file),
                GetFileResult::ErrNotInManifest => unreachable!(),
                GetFileResult::ErrIo(err) => Err(err),
            }
        },
        LoadFileResult::ErrIo(err) => Err(err)
    };
    let file = match result {
        Ok(ok) => ok,
        Err(err) => {
            Diagnostic::from(err)
                .message(("failed to open module at path `{}`", [
                    module_path.display().into(),
                ]))
                .report(issues);
            return None;
        },
    };
    let ast = ast::parse::from_file(issues, file.as_ref());
    let hir = hir::lower::from_ast(issues, &ast);
    // write to cache
    let cache_path = cache_dir.join(format!("{}.bin", file.id));
    let _ = fs::create_dir_all(cache_dir);
    if let Ok(mut file) = fs::File::create(cache_path) {
        let config = bincode::config::standard();
        let _ = bincode::encode_into_std_write(&hir, &mut file, config);
    }
    Some(hir)
}

/// Takes the HIR modules of a Cosy package, and uses them to compile the
/// complete Cosy ASM for the package.
///
/// Reports any errors to `issues`.
pub fn build_package_casm(
    _issues : &mut IssueManager,
    _cache_dir : &Path,
) -> Option<casm::Package> {
    Some(casm::Package)
}

/// Takes the Cosy ASM of a package, and uses it to emit LLVM bitcode. Both
/// the `.bc` bitcode and `.o` files are written to the cache directory.
///
/// Returns the path of the `.o` file.
///
/// Reports any errors to `issues`.
pub fn build_package_llvm(
    issues : &mut IssueManager,
    cache_dir : &Path,
    _casm : &casm::Package,
) -> Option<PathBuf> {
    None
}

/// Uses `clang` to link `.o` files into an executable file.
pub fn link_program(
    _issues : &mut IssueManager,
    _o_files : &[PathBuf],
    _out_path : &Path,
) -> bool {
    false
}

/// Takes a path, returning the name of the package to compile, and
/// its root Cosy file.
pub fn find_package_root(
    issues : &mut IssueManager,
    path : &Path
) -> Option<(String, PathBuf)> {
    let is_dir = match fs::metadata(path) {
        Ok(meta) => meta.is_dir(),
        Err(err) => {
            Diagnostic::from(err)
                .message(("failed to find package at path `{}`", [
                    path.display().into()
                ]))
                .report(issues);
            return None;
        }
    };
    let (os_name, entry);
    if is_dir {
        // look for main.cy
        os_name = path.file_name().unwrap();
        let mut entry_ = path.to_owned();
        entry_.push("main.cy");
        entry = entry_;
    } else {
        // we're a file! use the filename
        os_name = path.file_stem().unwrap();
        entry = path.to_owned();
    }
    let name = if let Some(name) = os_name.to_str() {
        name.to_owned()
    } else {
        Diagnostic::error()
            .message(("invalid package name {} is not a valid UTF-8 name", [
                format!("{:?}", os_name).into()
            ]))
            .report(issues);
        return None;
    };
    Some((name, entry))
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Config {
    Release,
    Debug,
}

impl Config {
    fn as_str(&self) -> &'static str {
        match self {
            Config::Release => "release",
            Config::Debug => "debug",
        }
    }
}

/// Returns the path of the default cache directory for this configuration.
pub fn default_cache(config : Config) -> PathBuf {
    let mut cache_dir = PathBuf::new();
    cache_dir.push("build");
    cache_dir.push(config.as_str());
    cache_dir.push("cache");
    cache_dir
}