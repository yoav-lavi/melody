pub mod consts;
pub mod macros;
pub mod output;
pub mod utils;

use clap::Parser;
use colored::control::{ShouldColorize, SHOULD_COLORIZE};
use consts::COMMAND_MARKER;
use melody_compiler::{compiler, errors::ParseError};
use output::{
    print_output, print_repl_welcome, print_source_line, prompt, report_clear, report_exit,
    report_missing_path, report_no_lines_to_print, report_nothing_to_redo, report_nothing_to_undo,
    report_parse_error, report_read_file_error, report_read_input_error, report_redo,
    report_repl_parse_error, report_source, report_undo, report_unrecognized_command,
    report_write_file_error,
};
use std::fs::{read_to_string, write};
use utils::{exit, read_input, ExitCode};

#[derive(Parser, Debug)]
#[clap(about, version, author)]
struct Args {
    #[clap(value_name = "INPUT_FILE_PATH", help = "Read from a file")]
    input_file_path: Option<String>,
    #[clap(
        short = 'o',
        long = "output",
        value_name = "OUTPUT_FILE_PATH",
        help = "Write to a file"
    )]
    output_file_path: Option<String>,
    #[clap(short = 'n', long = "no-color", help = "Print output with no color")]
    no_color_output: bool,
    #[clap(short = 'r', long = "repl", help = "Start the Melody REPL")]
    start_repl: bool,
}

enum CliError {
    MissingPath,
    ReadFileError(String),
    ParseError(ParseError),
    WriteFileError(String),
    ReadInputError,
}

fn main() {
    ShouldColorize::from_env();

    match cli() {
        Ok(_) => exit(ExitCode::Ok),
        Err(error) => {
            match error {
                CliError::MissingPath => report_missing_path(),
                CliError::ReadFileError(path) => report_read_file_error(path),
                CliError::WriteFileError(output_file_path) => {
                    report_write_file_error(output_file_path)
                }
                CliError::ParseError(parse_error) => report_parse_error(parse_error.message),
                CliError::ReadInputError => report_read_input_error(),
            }
            exit(ExitCode::Error)
        }
    };
}

fn cli() -> Result<(), CliError> {
    let args = Args::parse();

    let Args {
        start_repl,
        input_file_path,
        output_file_path,
        no_color_output,
    } = args;

    if no_color_output {
        SHOULD_COLORIZE.set_override(false);
    }

    if start_repl {
        return repl();
    }

    let input_file_path = input_file_path.ok_or(CliError::MissingPath)?;

    let source = read_to_string(input_file_path.clone())
        .map_err(|_| CliError::ReadFileError(input_file_path))?;

    let compiler_output = compiler(&source).map_err(CliError::ParseError)?;

    match output_file_path {
        Some(output_file_path) => {
            write(&output_file_path, compiler_output)
                .map_err(|_| CliError::WriteFileError(output_file_path))?;
        }
        None => {
            print_output(compiler_output);
        }
    }

    Ok(())
}

fn repl() -> Result<(), CliError> {
    print_repl_welcome();

    let mut valid_lines: Vec<String> = Vec::new();
    let mut redo_lines: Vec<String> = Vec::new();

    'repl: loop {
        prompt();

        let input = read_input().map_err(|_| CliError::ReadInputError)?;

        if input.starts_with(COMMAND_MARKER) {
            match input.as_str() {
                format_command!("u", "undo") => {
                    if valid_lines.is_empty() {
                        report_nothing_to_undo();
                    } else {
                        report_undo(false);

                        let latest = valid_lines.pop().unwrap();
                        redo_lines.push(latest);

                        if !valid_lines.is_empty() {
                            let source = &valid_lines.join("\n");
                            let raw_output = compiler(source);
                            let output = raw_output.unwrap();

                            print_output(format!("{output}\n"));
                        }
                    }
                }
                format_command!("r", "redo") => {
                    if redo_lines.is_empty() {
                        report_nothing_to_redo();
                    } else {
                        report_redo();

                        let latest = redo_lines.pop().unwrap();
                        valid_lines.push(latest);

                        let source = &valid_lines.join("\n");
                        let raw_output = compiler(source);
                        let output = raw_output.unwrap();

                        print_output(format!("{output}\n"));
                    }
                }
                format_command!("s", "source") => {
                    if valid_lines.is_empty() {
                        report_no_lines_to_print();
                    } else {
                        report_source();

                        for (line_index, line) in valid_lines.iter().enumerate() {
                            print_source_line(line_index + 1, String::from(line));
                        }

                        println!();
                    }
                }
                format_command!("c", "clear") => {
                    report_clear();

                    valid_lines.clear();
                    redo_lines.clear();
                }
                format_command!("e", "exit") => {
                    report_exit();

                    return Ok(());
                }
                _ => report_unrecognized_command(input.trim().to_owned()),
            }

            continue 'repl;
        }

        if input.is_empty() {
            let source = &valid_lines.join("\n");
            let raw_output = compiler(source);
            let output = raw_output.unwrap();

            print_output(format!("{output}\n"));

            continue 'repl;
        }

        valid_lines.push(input);

        let source = &valid_lines.join("\n");
        let raw_output = compiler(source);

        if let Err(error) = raw_output {
            let ParseError { message } = error;

            report_repl_parse_error(message);

            valid_lines.pop();

            continue 'repl;
        }

        redo_lines.clear();

        let output = raw_output.unwrap();

        print_output(format!("{output}\n"))
    }
}
