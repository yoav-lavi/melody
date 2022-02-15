use colored::Colorize;

pub fn report_group_end_warning(line: u16) {
    println!(
        "{} {} {} {} {}\n",
        "Warning:".bright_yellow(),
        "Ignoring".bright_yellow(),
        "\"}\"".bright_blue(),
        "on line".bright_yellow(),
        line.to_string().bright_blue()
    );
}

pub fn report_parse_error(source: &str, line_source: &str, line: u16) {
    eprintln!(
        "{} {} {} {} {}{}{}{}",
        "Error:".bright_red(),
        "Unable to parse".bright_red(),
        format!("\"{source}\"").bright_blue(),
        "on line".bright_red(),
        line.to_string().bright_blue(),
        ":\n\n".bright_red(),
        format!("{line}: ").dimmed(),
        line_source
    );
}

pub fn report_read_file_error(path: &str) {
    eprintln!(
        "{} {} {}",
        "Error:".bright_red(),
        "Unable read file at path".bright_red(),
        format!("\"{path}\"").bright_blue(),
    );
}

pub fn report_write_file_error(path: &str) {
    eprintln!(
        "{} {} {}",
        "Error:".bright_red(),
        "Unable write file at path".bright_red(),
        format!("\"{path}\"").bright_blue(),
    );
}

pub fn print_output(output: String) {
    println!("{}", output);
}

pub fn print_output_pretty(output: String) {
    println!("{}", output.bright_blue());
}
