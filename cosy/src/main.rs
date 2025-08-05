mod cli;

use libcosyc::{
    Session,
    reporting::{ Renderer, log::LogRenderer }
};
use std::{ io, process::ExitCode };

pub fn main() -> ExitCode {
    let mut sess = Session::default();
    cli::execute(&mut sess);
    let mut renderer = LogRenderer::default();
    let stderr = &mut io::stderr();
    if let Err(err) = renderer.render_session(stderr, &sess) {
        println!(
            "ENCOUNTERED AN UNEXPECTED ERROR WHEN REPORTING ERRORS:\n{}",
            err
        );
        return ExitCode::from(2);
    }
    if sess.issues.has_errors() {
        return ExitCode::from(1);
    }
    ExitCode::SUCCESS
}