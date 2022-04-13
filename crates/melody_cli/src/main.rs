#![forbid(unsafe_code)]
#![warn(clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]

mod compile;
mod completions;
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
use completions::generate_completions;
use consts::STDIN_MARKER;
use errors::{handle_error, CliError};
use output::{report_error, report_info};
use repl::repl;
use std::process;
use types::{Args, Streams};

fn main() {
    ShouldColorize::from_env();

    match try_main() {
        Ok(_) => process::exit(exitcode::OK),
        Err(error) => handle_error(&error),
    };
}

fn try_main() -> anyhow::Result<()> {
    let Args {
        start_repl,
        input_file_path,
        output_file_path,
        no_color_output,
        completions,
    } = Args::parse();

    if no_color_output {
        SHOULD_COLORIZE.set_override(false);
    }

    if let Some(completions) = completions {
        generate_completions(&completions);
        return Ok(());
    }

    let input_file_path = input_file_path.unwrap_or_else(|| STDIN_MARKER.to_owned());

    argument_env_validation(start_repl, &input_file_path)?;

    if start_repl {
        return repl();
    }

    compile_file(&input_file_path, output_file_path)?;

    Ok(())
}

fn argument_env_validation(start_repl: bool, input_file_path: &str) -> anyhow::Result<()> {
    let streams = Streams::new();

    if streams.any_pipe() && start_repl {
        return Err(CliError::ReplWithPipe.into());
    }
    if !streams.stdin && !start_repl && input_file_path == STDIN_MARKER {
        return Err(CliError::StdinWithoutPipe.into());
    }

    Ok(())
}
