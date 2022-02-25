pub mod output;

use clap::Parser;
use melody_compiler::{compiler, ParseError};
use output::{
    print_output, print_output_pretty, print_repl_welcome, prompt, report_exit,
    report_missing_path, report_nothing_to_redo, report_nothing_to_undo, report_parse_error,
    report_read_file_error, report_redo, report_repl_parse_error, report_undo,
    report_write_file_error,
};
use std::fs::{read_to_string, write};
use std::io::Write;

#[derive(Parser, Debug)]
#[clap(about, version, author)]
struct Args {
    path: Option<String>,
    #[clap(short, long)]
    file: Option<String>,
    #[clap(short, long)]
    no_color: bool,
    #[clap(short, long)]
    repl: bool,
}

fn main() {
    let args = Args::parse();

    let raw_file_path = args.path;

    let repl = args.repl;

    if repl {
        return run_repl();
    }

    if raw_file_path.is_none() {
        report_missing_path();
        std::process::exit(1);
    }

    let file_path = raw_file_path.unwrap();

    let source = match read_to_string(file_path.clone()) {
        Ok(source) => source,
        Err(_) => {
            report_read_file_error(file_path);
            std::process::exit(1);
        }
    };

    let raw_output = compiler(&source);

    if let Err(error) = raw_output {
        let ParseError {
            token,
            line,
            line_index,
        } = error;

        let line_number = line_index + 1;
        report_parse_error(token, line, line_number);

        std::process::exit(1);
    }

    let output = raw_output.unwrap();

    let output_file_path = &args.file;
    let no_color = args.no_color;

    match output_file_path {
        Some(output_file_path) => {
            let write_result = write(output_file_path, output);
            if write_result.is_err() {
                report_write_file_error(output_file_path);
                std::process::exit(1);
            };
        }
        None => {
            if no_color {
                print_output(output);
            } else {
                print_output_pretty(output);
            }
        }
    }
}

fn run_repl() {
    print_repl_welcome();

    let mut store: Vec<String> = Vec::new();
    let mut redo_store: Vec<String> = Vec::new();

    loop {
        prompt();
        let _ = std::io::stdout().flush();
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        match input.as_str() {
            ".u\n" => {
                let length = store.len();
                if length == 0 {
                    report_nothing_to_undo();
                    continue;
                } else if length == 1 {
                    let latest = store.pop().unwrap();
                    redo_store.push(latest);
                    report_undo(true);
                    continue;
                }
                let latest = store.pop().unwrap();
                redo_store.push(latest);
                report_undo(false);
            }
            ".e\n" => {
                report_exit();
                std::process::exit(0);
            }
            ".r\n" => {
                let length = redo_store.len();
                if length == 0 {
                    report_nothing_to_redo();
                    continue;
                } else {
                    let latest = redo_store.pop().unwrap();
                    store.push(latest);
                    report_redo();
                }
            }
            _ => {
                store.push(input);
            }
        }

        let source = &store.join("\n");
        let raw_output = compiler(source);

        if let Err(error) = raw_output {
            let ParseError {
                token,
                line: _,
                line_index: _,
            } = error;

            report_repl_parse_error(token);

            store.pop();
            continue;
        }

        redo_store.clear();

        let output = raw_output.unwrap();
        print_output_pretty(format!("{output}\n"))
    }
}
