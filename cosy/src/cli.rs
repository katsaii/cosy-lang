mod cmd_run;
mod cmd_debug_lex;

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
    #[command(subcommand)]
    Debug(CosycCommandDebug),
}

/// Debug output utilities; can be used to chase bugs or satisfy curiosity.
#[derive(Subcommand)]
enum CosycCommandDebug {
    Lex(cmd_debug_lex::Args),
}

#[allow(dead_code)]
pub(super) fn execute() -> () {
    let cosyc_args = Cosyc::parse();
    return match cosyc_args.command {
        CosycCommand::Run(args) => cmd_run::execute(args),
        CosycCommand::Debug(debug_cmd) => match debug_cmd {
            CosycCommandDebug::Lex(args) => cmd_debug_lex::execute(args),
        },
    };
}