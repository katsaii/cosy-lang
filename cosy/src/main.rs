mod cli;

use libcosyc::{
    Session,
    reporting::{ Renderer, log::LogRenderer }
};
use std::io;

pub fn main() {
    let mut sess = Session::default();
    cli::execute(&mut sess);
    let mut renderer = LogRenderer;
    let stderr = &mut io::stderr();
    if let Err(err) = renderer.render_session(stderr, &sess) {
        println!(
            "ENCOUNTERED AN UNEXPECTED ERROR WHEN REPORTING ERRORS:\n{}",
            err
        );
    }
}