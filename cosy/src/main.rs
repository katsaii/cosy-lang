mod cli;

use std::process::ExitCode;
use libcosyc::Session;

pub fn main() -> ExitCode {
    let mut sess = Session::default();
    cli::execute(&mut sess)
}