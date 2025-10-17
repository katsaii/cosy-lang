use std::path::{ Path, PathBuf };
use std::fs;

use crate::src::SourceMap;
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
    _files : &mut SourceMap,
    _issues : &mut IssueManager,
    _cache_dir : &Path,
    _module_path : &Path,
) -> Option<hir::Module> {
    Some(hir::Module)
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