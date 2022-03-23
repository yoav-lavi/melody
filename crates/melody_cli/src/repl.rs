use crate::consts::COMMAND_MARKER;
use crate::errors::CliError;
use crate::output::{
    print_output_repl, print_repl_welcome, print_source_line, prompt, report_clear, report_exit,
    report_no_lines_to_print, report_nothing_to_redo, report_nothing_to_undo, report_redo,
    report_repl_error, report_source, report_undo, report_unrecognized_command,
};
use crate::utils::read_input;
use melody_compiler::compiler;

pub fn repl() -> anyhow::Result<()> {
    print_repl_welcome();

    let mut valid_lines: Vec<String> = Vec::new();
    let mut redo_lines: Vec<String> = Vec::new();

    'repl: loop {
        prompt();

        let input = read_input().map_err(|_| CliError::ReadInputError)?;

        if input.starts_with(COMMAND_MARKER) {
            match input.as_str() {
                format_command!("u", "undo") => {
                    if valid_lines.is_empty() {
                        report_nothing_to_undo();
                    } else {
                        report_undo(false);

                        let latest = valid_lines.pop().unwrap();
                        redo_lines.push(latest);

                        if !valid_lines.is_empty() {
                            let source = &valid_lines.join("\n");
                            let raw_output = compiler(source);
                            let output = raw_output.unwrap();

                            print_output_repl(&output);
                        }
                    }
                }
                format_command!("r", "redo") => {
                    if redo_lines.is_empty() {
                        report_nothing_to_redo();
                    } else {
                        report_redo();

                        let latest = redo_lines.pop().unwrap();
                        valid_lines.push(latest);

                        let source = &valid_lines.join("\n");
                        let raw_output = compiler(source);
                        let output = raw_output.unwrap();

                        print_output_repl(&output);
                    }
                }
                format_command!("s", "source") => {
                    if valid_lines.is_empty() {
                        report_no_lines_to_print();
                    } else {
                        report_source();

                        for (line_index, line) in valid_lines.iter().enumerate() {
                            print_source_line(line_index + 1, line);
                        }

                        println!();
                    }
                }
                format_command!("c", "clear") => {
                    report_clear();

                    valid_lines.clear();
                    redo_lines.clear();
                }
                format_command!("e", "exit") => {
                    report_exit();

                    return Ok(());
                }
                _ => report_unrecognized_command(input.trim()),
            }

            continue 'repl;
        }

        if input.is_empty() {
            let source = &valid_lines.join("\n");
            let raw_output = compiler(source);
            let output = raw_output.unwrap();

            print_output_repl(&output);

            continue 'repl;
        }

        valid_lines.push(input);

        let source = &valid_lines.join("\n");
        let raw_output = compiler(source);

        if let Err(error) = raw_output {
            report_repl_error(&error.to_string());

            valid_lines.pop();

            continue 'repl;
        }

        redo_lines.clear();

        let output = raw_output.unwrap();

        print_output_repl(&output);
    }
}
