mod cmd_run;

use clap::{ Parser, Subcommand };

/// The Cosy compiler! /(.@ w @.) b
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cosyc {
    /// The Cosy command to execute.
    #[command(subcommand)]
    command: CosycCommand,
}

#[derive(Subcommand)]
enum CosycCommand {
    /// Builds the package and immediately runs its entrypoint.
    Run {
        /// Path to the package to build (defaults to the working directory):
        ///  * If the path is a `.cosy` file, then that file will act as the entrypoint.
        ///  * If the path is a directory, then a file named `main.cosy` will be used as the entrypoint.
        #[arg(verbatim_doc_comment)]
        package_path : Option<String>,
    },
}

pub fn execute() -> () {
    let cosyc_args = Cosyc::parse();
    return match cosyc_args.command {
        CosycCommand::Run { package_path } => cmd_run::execute(package_path), 
    };
}