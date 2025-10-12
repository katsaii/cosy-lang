pub mod src;
pub mod pretty;
pub mod error;
pub mod ir;

use std::path::{ Path, PathBuf };

use crate::src::SourceMap;
use crate::error::IssueManager;
#[allow(unused_imports)] use crate::ir::{ ast, hir, casm };

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
    _issues : &mut IssueManager,
    _cache_dir : &Path,
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