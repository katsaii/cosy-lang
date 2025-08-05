mod cli;

use libcosyc::{
    Session,
    reporting::{ Renderer, log::LogRenderer }
};
use std::{ env, io, io::IsTerminal, process::ExitCode };

pub fn main() -> ExitCode {
    let mut sess = Session::default();
    cli::execute(&mut sess);
    let (mut stderr, mut renderer) = get_renderer();
    if let Err(err) = renderer.render_session(&mut stderr, &sess) {
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

fn get_renderer() -> (io::Stderr, LogRenderer) {
    let stderr = io::stderr();
    let use_colour = 'blk: {
        if !stderr.is_terminal() {
            break 'blk false;
        }
        if let Ok(val) = env::var("CLICOLOR_FORCE") {
            if val != "0" {
                break 'blk true;
            }
        }
        if env::var("NO_COLOR").is_ok() {
            break 'blk false;
        }
        if let Ok(val) = env::var("CLICOLOR_FORCE") {
            break 'blk val != "0";
        }
        true
    };
    (stderr, LogRenderer::new(use_colour))
}