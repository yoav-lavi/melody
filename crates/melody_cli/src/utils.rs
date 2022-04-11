use crate::errors::CliError;
use std::fs::write;
use std::io::{self, stdin, stdout, Write};

pub fn read_input() -> io::Result<String> {
    stdout().flush()?;
    let mut input = String::new();
    stdin().read_line(&mut input)?;

    Ok(String::from(input.trim_end()))
}

pub fn write_output_to_file(output_file_path: &str, compiler_output: &str) -> anyhow::Result<()> {
    write(output_file_path, compiler_output)
        .map_err(|_| CliError::WriteFileError(output_file_path.to_owned()))?;

    Ok(())
}
