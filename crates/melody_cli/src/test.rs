use std::fs::read_to_string;

use regress::Regex;

use crate::{
    errors::CliError,
    output::{report_test_result, report_test_result_file},
};

pub fn test_input(regex: &str, input: &str) -> anyhow::Result<()> {
    let regex = Regex::new(regex).map_err(|error| {
        CliError::CompileRegex(error.to_string().to_lowercase(), regex.to_string())
    })?;
    let matched = regex.find(input).is_some();
    report_test_result(matched, input);

    Ok(())
}

pub fn test_input_file(regex: &str, file_path: &str) -> anyhow::Result<()> {
    let input =
        read_to_string(file_path).map_err(|_| CliError::ReadFileError(file_path.to_owned()))?;
    let regex = Regex::new(regex).map_err(|error| {
        CliError::CompileRegex(error.to_string().to_lowercase(), regex.to_string())
    })?;
    let matched = regex.find(&input).is_some();
    report_test_result_file(matched, file_path);

    Ok(())
}
