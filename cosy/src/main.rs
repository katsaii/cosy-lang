mod cli;

use libcosyc::Session;

pub fn main() {
    let mut sess = Session::default();
    cli::execute(&mut sess);
    if let Some(stats) = sess.issues.error_stats() {
        println!("{:?}", stats);
    }
}