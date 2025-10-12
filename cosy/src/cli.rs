mod cmd_build;
mod cmd_debug_lex;
mod cmd_debug_parse;
mod cmd_debug_error;

use std::io;
use clap::{ Parser, Subcommand, Args };
use libcosyc::{ pretty, pretty::PrettyPrinter };

/// The Cosy compiler! /(.@ w @.) b
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cosyc {
    /// Whether to disable printing to the output window in colour.
    #[arg(long="no-colour")]
    use_no_colour : bool,
    /// The Cosy command to execute.
    #[command(subcommand)]
    command : CosycCommand,
}

type PrinterTy = PrettyPrinter<io::Stderr>;

struct CommonArgs {
    printer : PrinterTy,
}

#[derive(Subcommand)]
enum CosycCommand {
    Build(cmd_build::Args),
    #[command(subcommand)]
    Debug(CosycCommandDebug),
}

/// Debug output utilities; can be used to chase bugs or satisfy curiosity.
#[derive(Subcommand)]
enum CosycCommandDebug {
    Lex(cmd_debug_lex::Args),
    Parse(cmd_debug_parse::Args),
    Error(cmd_debug_error::Args),
}

pub(super) fn execute() {
    let cosyc_args = Cosyc::parse();
    let common_args = CommonArgs {
        printer : pretty::from_term(io::stderr(), !cosyc_args.use_no_colour),
    };
    match cosyc_args.command {
        CosycCommand::Build(args) => cmd_build::execute(common_args, args),
        CosycCommand::Debug(debug_cmd) => match debug_cmd {
            CosycCommandDebug::Lex(args) => cmd_debug_lex::execute(common_args, args),
            CosycCommandDebug::Parse(args) => cmd_debug_parse::execute(common_args, args),
            CosycCommandDebug::Error(args) => cmd_debug_error::execute(common_args, args),
        },
    }
}