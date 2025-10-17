use std::path::PathBuf;
use std::fs;

use libcosyc::src::{ SourceMap, LoadManifestResult, SaveManifestResult };
use libcosyc::error::{ cli, IssueManager };
use libcosyc::build;

/// Builds the package and immediately runs its entrypoint.
#[derive(super::Args)]
pub(super) struct Args {
    /// The path to the cache directory. Defaults to `build/<config>/cache`.
    #[arg(short, long="cache")]
    cache_dir : Option<PathBuf>,
    /// Path to the package to build (defaults to the working directory):
    ///  * If the path is a `.cy` file, then that file will act as the entrypoint.
    ///  * If the path is a directory, then a file named `main.cy` will be used as the entrypoint.
    #[arg(verbatim_doc_comment)]
    package_path : PathBuf,
}

// ...oh how i yearn for #![feature(try_blocks)]
macro_rules! labelled_try {
    ($label:lifetime, $computation:expr) => {
        match $computation {
            Some(x) => x,
            None => break $label,
        }
    }
}

pub(super) fn execute(mut cargs : super::CommonArgs, args : Args) {
    let cache = args.cache_dir.unwrap_or_else(|| {
        build::default_cache(build::Config::Debug)
    });
    fs::create_dir_all(&cache).unwrap();
    let cache_manifest = cache.as_path().join("manifest.bin");
    let mut issues = IssueManager::default();
    let mut files = match SourceMap::load_from_path(&cache_manifest) {
        LoadManifestResult::Ok(ok) => ok,
        _ => SourceMap::new(),
    };
    'task: {
        let (name, root) = labelled_try!('task, build::find_package_root(
            &mut issues,
            &args.package_path,
        ));
        let cache_package = cache.as_path().join(&name);
        let _hir = labelled_try!('task, build::build_module(
            &mut files,
            &mut issues,
            &cache_package,
            &root,
        ));
        println!("{:?}", _hir);
        let casm = labelled_try!('task, build::build_package_casm(
            &mut issues,
            &cache_package,
        ));
        let bc_path = labelled_try!('task, build::build_package_llvm(
            &mut issues,
            &cache_package,
            &casm,
        ));
        let cache_bin = cache_package.as_path().join(&name);
        if !build::link_program(
            &mut issues,
            &[bc_path],
            &cache_bin,
        ) { break 'task }
    }
    cli::write_errors(&mut cargs.printer, &mut files, &mut issues).unwrap();
}