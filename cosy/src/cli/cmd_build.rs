use std::fs;
use std::process::ExitCode;
use std::path::{ Path, PathBuf };

/// Builds the package and immediately runs its entrypoint.
#[derive(super::Args)]
pub(super) struct Args {
    /// Path to the package to build (defaults to the working directory):
    ///  * If the path is a `.cy` file, then that file will act as the entrypoint.
    ///  * If the path is a directory, then a file named `main.cy` will be used as the entrypoint.
    #[arg(verbatim_doc_comment)]
    package_path : PathBuf,
}

pub(super) fn execute(
    _args_other : super::CommonArgs,
    _args : Args,
) {
    //let mut sess = Session::new();
    //if let Some((name, entry)) = find_package_root(&mut sess, &args.package_path) {
    //    build_package(&mut sess, name, entry);
    //}
    //err.submit(&sess);
}
/*
fn find_package_root(sess : &mut Session, path : &Path) -> Option<(String, PathBuf)> {
    let is_dir = match fs::metadata(path) {
        Ok(meta) => meta.is_dir(),
        Err(err) => {
            Diagnostic::error()
                .message(("failed to compile package with root `{}`", [path.display().into()]))
                .note(("{}", [err.into()]))
                .report(&mut sess.issues);
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
            .message(("invalid package name {} is not a valid UTF-8 name", [format!("{:?}", os_name).into()]))
            .report(&mut sess.issues);
        return None;
    };
    Some((name, entry))
}

fn build_package(sess : &mut Session, _name : String, entry : PathBuf) -> Option<()> {
    let hir = build_hir_module(sess, &entry)?;
    println!("HIR:\n{:?}", hir);
    let casm = build_casm_package(sess, &hir)?;
    println!("CASM:\n{:?}", casm);
    build_llvm_target(sess, &casm)?;
    Some(())
}

fn build_hir_module(sess : &mut Session, path : &Path) -> Option<hir::Module> {
    let file_data = match sess.manifest.load(path) {
        Ok(ok) => ok,
        Err(err) => {
            Diagnostic::error()
                .message(("failed to open file `{}`", [path.display().into()]))
                .note(("{}", [err.into()]))
                .report(&mut sess.issues);
            return None;
        },
    };
    let ast = Parser::parse(&mut sess.issues, &file_data);
    if sess.issues.has_errors() {
        return None;
    }
    let hir = Ast2Hir::lower(&mut sess.issues, &ast);
    if sess.issues.has_errors() {
        return None;
    }
    Some(hir)
}

fn build_casm_package(sess : &mut Session, hir : &hir::Module) -> Option<casm::Package> {
    let casm = Hir2Casm::lower(&mut sess.issues, &hir);
    if sess.issues.has_errors() {
        return None;
    }
    Some(casm)
}

fn build_llvm_target(sess : &mut Session, casm : &casm::Package) -> Option<()> {
    llvm::emit_to_file(&mut sess.issues, casm, "cosy.bc".as_ref());
    if sess.issues.has_errors() {
        return None;
    }
    Some(())
}
*/