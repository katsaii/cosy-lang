mod cli;

use std::process::ExitCode;

use libcosyc_new as cosy;

pub fn main() -> ExitCode {
    let mut sess = cosy::build::Session::new();
    if let cosy::src::LoadFileResult::Ok(file) = sess.files.load_file_if_new_or_modified("examples/main.cy".as_ref()) {
        let loc = file.location(&cosy::src::Span::new(1..26));
        cosy::error::Diagnostic::error()
            .message("something bad")
            .label((loc, "whoops"))
            .note("guh")
            .report(&mut sess.issues);
    }
    sess.print_errors(false, true);

    cli::execute();
    ExitCode::SUCCESS
}