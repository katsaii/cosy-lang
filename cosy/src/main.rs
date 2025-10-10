mod cli;

use std::process::ExitCode;

use libcosyc_new as cosy;

pub fn main() -> ExitCode {
    let mut sess = cosy::build::Session::new();
    if let cosy::src::LoadFileResult::Ok(file) = sess.files.load_file_if_new_or_modified("examples/main.cy".as_ref()) {
        let mut printer = cosy::pretty::from_term(std::io::stdout(), true);
        let ast = cosy::ir::ast::parse::Parser::parse(&mut sess.issues, file.as_ref());
        cosy::ir::ast::debug_print_ast(&sess.files, &ast);
    }

    cli::execute();
    ExitCode::SUCCESS
}