use libcosyc::Session;

/// Builds the package and immediately runs its entrypoint.
#[derive(super::Args)]
pub(super) struct Args {
    /// Path to the package to build (defaults to the working directory):
    ///  * If the path is a `.cy` file, then that file will act as the entrypoint.
    ///  * If the path is a directory, then a file named `main.cy` will be used as the entrypoint.
    #[arg(verbatim_doc_comment)]
    package_path : Option<String>,
}

pub(super) fn execute(err : &mut super::ErrorReporter, _args : Args) {
    let sess = Session::new();
    println!("Hello, world!");
    err.submit(&sess);
}