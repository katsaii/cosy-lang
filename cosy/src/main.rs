mod cli;

use std::process::ExitCode;

pub fn main() -> ExitCode {
    cli::execute();
    ExitCode::SUCCESS
}