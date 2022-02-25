use colored::Colorize;

pub fn report_read_file_error(path: String) {
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

pub fn report_repl_parse_error(source: String) {
    eprintln!(
        "{} {} {}",
        "Error:".bright_red(),
        "Unable to parse".bright_red(),
        format!("\"{source}\"").bright_blue(),
    );
}

pub fn report_parse_error(source: String, line_source: String, line: usize) {
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

pub fn print_repl_welcome() {
    println!(
        "{}\n\n{}\n\n{}\n{}\n{}\n",
        "Melody REPL v0.7.0".bright_green(),
        "Commands:".bright_green(),
        format_args!("- {} - {}", ".u".bright_blue(), "undo".bright_green()),
        format_args!(
            "- {} - {}",
            ".e".bright_blue(),
            "exits the REPL".bright_green()
        ),
        format_args!("- {} - {}", ".r".bright_blue(), "redo".bright_green()),
    );
}

pub fn report_nothing_to_undo() {
    eprintln!("{}", "nothing to undo\n".bright_red());
}

pub fn prompt() {
    print!("{}", "~ ".bright_blue());
}

pub fn report_undo(newline: bool) {
    println!(
        "{}{}",
        "undo".bright_green(),
        if newline { "\n" } else { "" }
    );
}

pub fn report_exit() {
    println!("{}", "exiting".bright_green());
}

pub fn report_nothing_to_redo() {
    eprintln!("{}", "nothing to redo\n".bright_red());
}

pub fn report_redo() {
    println!("{}", "redo".bright_green());
}

pub fn report_missing_path() {
    eprintln!(
        "{}",
        "Error: invalid arguments, please supply a path argument or use --repl".bright_red()
    );
}
