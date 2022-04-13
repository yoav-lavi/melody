use crate::consts::STDIN_MARKER;
use crate::errors::CliError;
use crate::utils::read_stdin;
use melody_compiler::compiler;
use std::fs::read_to_string;

fn read_file(path: &str) -> anyhow::Result<String> {
    let contents = read_to_string(path).map_err(|_| CliError::ReadFileError(path.to_owned()))?;
    Ok(contents)
}

pub fn compile_file(input_file_path: &str) -> anyhow::Result<String> {
    let source = if input_file_path == STDIN_MARKER {
        read_stdin()?
    } else {
        read_file(input_file_path)?
    };

    compiler(&source)
}
