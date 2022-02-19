pub mod output;

use clap::Parser;
use melody_compiler::{compiler, ParseError};
use output::{
    print_output, print_output_pretty, report_parse_error, report_read_file_error,
    report_write_file_error,
};
use std::fs::{read_to_string, write};

#[derive(Parser, Debug)]
#[clap(about, version, author)]
struct Args {
    path: String,
    #[clap(short, long)]
    file: Option<String>,
    #[clap(short, long)]
    no_color: bool,
}

fn main() {
    let args = Args::parse();

    let file_path = &args.path;

    let source = match read_to_string(file_path) {
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
