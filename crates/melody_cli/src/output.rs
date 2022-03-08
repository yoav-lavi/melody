use colored::Colorize;

pub fn report_read_file_error(path: &str) {
    eprintln!(
        "{} {} {}",
        "Error:".bright_red(),
        "unable read file at path".bright_red(),
        format!("\"{path}\"").bright_blue(),
    );
}

pub fn report_write_file_error(path: &str) {
    eprintln!(
        "{} {} {}",
        "Error:".bright_red(),
        "unable write file at path".bright_red(),
        format!("\"{path}\"").bright_blue(),
    );
}

pub fn print_output(output: &str) {
    print!("{}", output.bright_blue());
}

pub fn print_output_repl(output: &str) {
    println!("{}", output.bright_blue());
}

pub fn print_source_line(line_number: usize, line: &str) {
    println!(
        "{} {}",
        line_number.to_string().dimmed(),
        line.bright_blue()
    );
}

pub fn report_repl_parse_error(message: &str) {
    eprintln!("{} {}\n", "Error:".bright_red(), message.bright_red(),);
}

pub fn report_unrecognized_command(source: &str) {
    eprintln!(
        "{} {} {}\n",
        "Error:".bright_red(),
        "unrecognized command".bright_red(),
        format!("\"{source}\"").bright_blue(),
    );
}

pub fn report_parse_error(message: &str) {
    eprintln!("{} {}", "Error:".bright_red(), message.bright_red());
}

pub fn report_read_input_error() {
    eprintln!(
        "{} {}",
        "Error:".bright_red(),
        "unable to read input".bright_red(),
    );
}

pub fn print_repl_welcome() {
    println!(
        "{}\n\n{}\n\n{}\n{}\n{}\n{}\n{}\n",
        "Melody REPL v0.13.0".bright_green(),
        "Commands:".bright_green(),
        format_args!(
            "- {} - {}",
            ":u, :undo".bright_blue(),
            "undo the latest line".bright_green()
        ),
        format_args!(
            "- {} - {}",
            ":r, :redo".bright_blue(),
            "redo the latest undo".bright_green()
        ),
        format_args!(
            "- {} - {}",
            ":c, :clear".bright_blue(),
            "clear all REPL history and previous input".bright_green()
        ),
        format_args!(
            "- {} - {}",
            ":s, :source".bright_blue(),
            "print all previously entered lines".bright_green()
        ),
        format_args!(
            "- {} - {}",
            ":e, :exit".bright_blue(),
            "exit the REPL".bright_green()
        ),
    );
}

pub fn report_nothing_to_undo() {
    eprintln!("{}", "nothing to undo\n".bright_red());
}

pub fn report_no_lines_to_print() {
    eprintln!("{}", "no lines to print\n".bright_red());
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
    println!("{}", "exit".bright_green());
}

pub fn report_nothing_to_redo() {
    eprintln!("{}", "nothing to redo\n".bright_red());
}

pub fn report_redo() {
    println!("{}", "redo".bright_green());
}

pub fn report_clear() {
    println!("{}\n", "clear".bright_green());
}

pub fn report_source() {
    println!("{}", "source".bright_green());
}

pub fn report_missing_path() {
    eprintln!(
        "{}",
        "Error: invalid arguments, please supply a path argument or use --repl".bright_red()
    );
}
