mod cmd_build;
mod cmd_debug_lex;
mod cmd_debug_parse;
mod cmd_debug_error;

use std::{ cmp, env, io, io::IsTerminal, process::ExitCode };
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

pub(super) fn execute() -> ExitCode {
    let cosyc_args = Cosyc::parse();
    let mut reporter = ErrorReporter::new(cosyc_args.use_compact_errors, true);
    match cosyc_args.command {
        CosycCommand::Build(args) => cmd_build::execute(&mut reporter, args),
        CosycCommand::Debug(debug_cmd) => match debug_cmd {
            CosycCommandDebug::Lex(args) => cmd_debug_lex::execute(&mut reporter, args),
            CosycCommandDebug::Parse(args) => cmd_debug_parse::execute(&mut reporter, args),
            CosycCommandDebug::Error(args) => cmd_debug_error::execute(&mut reporter, args),
        },
    }
    reporter.finalise()
}

struct ErrorReporter {
    stderr : io::Stderr,
    use_colour : bool,
    use_compact_errors : bool,
    exit_code : u8,
}

impl ErrorReporter {
    fn new(use_compact_errors : bool, use_colour : bool) -> Self {
        let (stderr, supports_colour) = stderr_from_env();
        Self {
            stderr,
            use_colour : use_colour && supports_colour,
            use_compact_errors,
            exit_code : 0,
        }
    }

    fn submit(&mut self, sess : &Session) {
        let pretty = rep::PrettyPrinter::new(self.use_colour);
        let result = if self.use_compact_errors {
            rep::log::LogRenderer(pretty).render_session(&mut self.stderr, sess)
        } else {
            rep::cli::CliRenderer(pretty).render_session(&mut self.stderr, sess)
        };
        if let Err(err) = result {
            println!(
                "ENCOUNTERED AN UNEXPECTED ERROR WHEN REPORTING ERRORS:\n{}",
                err
            );
            self.exit_code = cmp::max(self.exit_code, 2);
        }
        if sess.issues.has_errors() {
            self.exit_code = cmp::max(self.exit_code, 1);
        }
    }

    fn finalise(self) -> ExitCode {
        if self.exit_code == 0 {
            ExitCode::SUCCESS
        } else {
            ExitCode::from(self.exit_code)
        }
    }
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