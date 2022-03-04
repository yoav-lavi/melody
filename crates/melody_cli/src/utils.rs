use crate::CliError;
use std::fs::write;
use std::io::{self, Write};

pub fn read_input() -> io::Result<String> {
    let _ = std::io::stdout().flush();
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;

    Ok(String::from(input.trim_end()))
}

pub enum ExitCode {
    Ok,
    Error,
}

pub fn exit(code: ExitCode) {
    match code {
        ExitCode::Ok => std::process::exit(0),
        ExitCode::Error => std::process::exit(1),
    }
}

pub fn write_output_to_file(
    output_file_path: String,
    compiler_output: String,
) -> Result<(), CliError> {
    write(&output_file_path, compiler_output)
        .map_err(|_| CliError::WriteFileError(output_file_path))?;

    Ok(())
}
