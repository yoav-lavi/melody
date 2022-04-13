use regress::Regex;

use crate::{errors::CliError, output::report_test_result};

pub fn test_input(regex: &str, input: &str) -> anyhow::Result<()> {
    let regex = Regex::new(regex).map_err(|error| {
        CliError::CompileRegex(error.to_string().to_lowercase(), regex.to_string())
    })?;
    let matched = regex.find(input).is_some();
    report_test_result(matched, input);

    Ok(())
}
