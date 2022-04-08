#![forbid(unsafe_code)]
#![warn(clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]

mod compile;
mod consts;
mod errors;
mod macros;
mod output;
mod repl;
mod types;
mod utils;

use clap::Parser;
use colored::control::{ShouldColorize, SHOULD_COLORIZE};
use compile::compile_file;
use errors::CliError;
use output::report_error;
use repl::repl;
use std::process;
use types::{Args, ExitCode};

fn main() {
    ShouldColorize::from_env();

    let exit_code = match try_main() {
        Ok(_) => ExitCode::Ok,
        Err(error) => {
            report_error(&error.to_string());
            ExitCode::Error
        }
    };

    process::exit(exit_code.into());
}

fn try_main() -> anyhow::Result<()> {
    let Args {
        start_repl,
        input_file_path,
        output_file_path,
        no_color_output,
    } = Args::parse();

    if no_color_output {
        SHOULD_COLORIZE.set_override(false);
    }

    if start_repl {
        return repl();
    }

    let input_file_path = input_file_path.ok_or(CliError::MissingPath)?;

    compile_file(input_file_path, output_file_path)?;

    Ok(())
}
