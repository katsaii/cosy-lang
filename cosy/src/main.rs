mod cli;

use std::process::ExitCode;

use libcosyc_new as cosy;

pub fn main() -> ExitCode {
    let mut sess = cosy::build::Session::new();
    if let cosy::src::LoadFileResult::Ok(file) = sess.files.load_file_if_new_or_modified("examples/main.cy".as_ref()) {
        let mut printer = cosy::pretty::from_env(true);
        cosy::ir::ast::parse::lex::debug_write_tokens(&mut printer, &file.src);
    }

    cli::execute();
    ExitCode::SUCCESS
}