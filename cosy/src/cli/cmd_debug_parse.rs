use libcosyc::parse::{ Parser, asg };
use libcosyc::Session;

/// Parses the contents of a file and prints its untyped ASG.
#[derive(super::Args)]
pub(super) struct Args {
    /// Path of the `.cy` file to parse.
    #[arg()]
    file_path : String,
}

pub(super) fn execute(sess : &mut Session, args : Args) {
    let file_id = match sess.files.load((&args.file_path).into()) {
        Ok(x) => x,
        Err(err) => {
            err.report(&mut sess.issues);
            return;
        },
    };
    let file = sess.files.get_file(file_id);
    let mut module = asg::Module::default();
    if Parser::parse(&mut sess.issues, file, &mut module) {
        println!("{:#?}", module);
    }
}