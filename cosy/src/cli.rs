mod cmd_run;

use clap::{ Parser, Subcommand, Args };

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
    Run(cmd_run::Args),
}

#[allow(dead_code)]
pub(super) fn execute() -> () {
    let cosyc_args = Cosyc::parse();
    return match cosyc_args.command {
        CosycCommand::Run(args) => cmd_run::execute(args),
    };
}