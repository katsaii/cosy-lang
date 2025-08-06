mod cmd_run;
mod cmd_debug_lex;
mod cmd_debug_error;

use std::{ env, io, io::IsTerminal, process::ExitCode };
use clap::{ Parser, Subcommand, Args };
use libcosyc::Session;
use libcosyc::reporting::{ self as rep, Renderer };

/// The Cosy compiler! /(.@ w @.) b
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cosyc {
    /// Whether to use a simplified error format.
    #[arg(long="compact-errors")]
    use_compact_errors : bool,
    /// The Cosy command to execute.
    #[command(subcommand)]
    command : CosycCommand,
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
    Error(cmd_debug_error::Args),
}

pub(super) fn execute(sess : &mut Session) -> ExitCode {
    let cosyc_args = Cosyc::parse();
    match cosyc_args.command {
        CosycCommand::Run(args) => cmd_run::execute(sess, args),
        CosycCommand::Debug(debug_cmd) => match debug_cmd {
            CosycCommandDebug::Lex(args) => cmd_debug_lex::execute(sess, args),
            CosycCommandDebug::Error(args) => cmd_debug_error::execute(sess, args),
        },
    }
    report_errors(sess, cosyc_args.use_compact_errors)
}

fn stderr_from_env() -> (io::Stderr, bool) {
    let stderr = io::stderr();
    let supports_colour = 'blk: {
        if !stderr.is_terminal() {
            break 'blk false;
        }
        if let Ok(val) = env::var("CLICOLOR_FORCE") {
            if val != "0" {
                break 'blk true;
            }
        }
        if env::var("NO_COLOR").is_ok() {
            break 'blk false;
        }
        if let Ok(val) = env::var("CLICOLOR_FORCE") {
            break 'blk val != "0";
        }
        true
    };
    (stderr, supports_colour)
}

fn report_errors(
    sess : &mut Session,
    use_compact_errors : bool,
) -> ExitCode {
    let (mut stderr, supports_colour) = stderr_from_env();
    let pretty = rep::PrettyPrinter::new(supports_colour);
    let result = if use_compact_errors {
        rep::log::LogRenderer(pretty).render_session(&mut stderr, sess)
    } else {
        rep::cli::CliRenderer(pretty).render_session(&mut stderr, sess)
    };
    if let Err(err) = result {
        println!(
            "ENCOUNTERED AN UNEXPECTED ERROR WHEN REPORTING ERRORS:\n{}",
            err
        );
        return ExitCode::from(2);
    }
    if sess.issues.has_errors() {
        return ExitCode::from(1);
    }
    ExitCode::SUCCESS
}