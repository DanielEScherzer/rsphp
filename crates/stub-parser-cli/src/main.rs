//! This crate defines a command line interface for interacting with the
//! [`rsphp_stub_parser`] crate. It is separate because it is not expected to
//! be needed by extensions after initial development.

use clap::{Parser, ValueEnum};
use std::path::PathBuf;

/// How should calls to `register_class_*` methods be verified for correctness?
///
/// Matches [`rsphp_stub_parser::VerificationMode`] but with `value_enum`
/// support.
#[derive(Copy, Clone, Default, Debug, ValueEnum)]
enum CLIVerificationMode {
    /// Always run assertions.
    Always,
    /// Only run assertions when debug assertions are enabled.
    #[default]
    DebugAssert,
    /// Don't add extra assertions (beyond the number of methods, which is
    /// always checked).
    Never,
}

/// Arguments for the CLI interface.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Stub file to parse.
    file: PathBuf,

    /// Output file to use, defaults to replacing `.stub.php` with `_arginfo.rs`.
    #[arg(short, long)]
    output: Option<PathBuf>,

    /// Verification options.
    #[arg(short, long, default_value_t, value_enum)]
    verification: CLIVerificationMode,

    /// Print rather than saving.
    #[arg(short, long)]
    dry_run: bool,
}

#[expect(clippy::print_stdout, reason = "CLI tool expects to be printing")]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let verification_mode = match args.verification {
        CLIVerificationMode::Always => rsphp_stub_parser::VerificationMode::Always,
        CLIVerificationMode::DebugAssert => rsphp_stub_parser::VerificationMode::DebugAssert,
        CLIVerificationMode::Never => rsphp_stub_parser::VerificationMode::Never,
    };
    if args.dry_run {
        let result = rsphp_stub_parser::process_stub(&args.file, verification_mode)?;
        println!("Result: {result}");
        Ok(())
    } else {
        rsphp_stub_parser::generate_arginfo(&args.file, args.output.as_ref(), verification_mode)
    }
}
