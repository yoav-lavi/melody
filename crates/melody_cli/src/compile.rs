use crate::errors::CliError;
use crate::output::print_output;
use crate::utils::write_output_to_file;
use melody_compiler::compiler;
use std::fs::read_to_string;

pub fn compile_file(
    input_file_path: String,
    output_file_path: Option<String>,
) -> anyhow::Result<()> {
    let source = read_to_string(input_file_path.clone())
        .map_err(|_| CliError::ReadFileError(input_file_path))?;

    let compiler_output =
        compiler(&source).map_err(|error| CliError::ParseError(error.to_string()))?;

    match output_file_path {
        Some(output_file_path) => write_output_to_file(output_file_path, compiler_output)?,
        None => print_output(&compiler_output),
    };

    Ok(())
}
