use crate::consts::STDIN_MARKER;
use crate::errors::CliError;
use crate::output::print_output;
use crate::utils::write_output_to_file;
use melody_compiler::compiler;
use std::fs::read_to_string;
use std::io::{self, Read};

fn read_stdin() -> anyhow::Result<String> {
    let mut buffer = String::new();
    io::stdin()
        .read_to_string(&mut buffer)
        .map_err(|_| CliError::ReadStdinError)?;
    Ok(buffer)
}

fn read_file(path: &str) -> anyhow::Result<String> {
    let contents = read_to_string(path).map_err(|_| CliError::ReadFileError(path.to_owned()))?;
    Ok(contents)
}

pub fn compile_file(input_file_path: &str, output_file_path: Option<String>) -> anyhow::Result<()> {
    let source = if input_file_path == STDIN_MARKER {
        read_stdin()?
    } else {
        read_file(input_file_path)?
    };

    let compiler_output =
        compiler(&source).map_err(|error| CliError::ParseError(error.to_string()))?;

    match output_file_path {
        Some(output_file_path) => write_output_to_file(&output_file_path, &compiler_output)?,
        None => print_output(&compiler_output),
    };

    Ok(())
}
