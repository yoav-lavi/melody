use crate::consts::COMMAND_MARKER;
use crate::errors::CliError;
use crate::output::{
    print_output_repl, print_repl_welcome, print_source_line, prompt, report_clear, report_exit,
    report_no_lines_to_print, report_nothing_to_redo, report_nothing_to_undo, report_redo,
    report_repl_error, report_source, report_undo, report_unrecognized_command,
};
use crate::types::NextLoop;
use crate::utils::read_input;
use melody_compiler::compiler;

pub fn repl() -> anyhow::Result<()> {
    print_repl_welcome();

    let mut valid_lines: Vec<String> = Vec::new();
    let mut redo_lines: Vec<String> = Vec::new();

    loop {
        match repl_loop(&mut valid_lines, &mut redo_lines)? {
            NextLoop::Continue => {}
            NextLoop::Exit => return Ok(()),
        }
    }
}

fn repl_loop(
    valid_lines: &mut Vec<String>,
    redo_lines: &mut Vec<String>,
) -> anyhow::Result<NextLoop> {
    let input = prompt_and_read()?;

    if input.starts_with(COMMAND_MARKER) {
        return Ok(repl_command(&input, valid_lines, redo_lines));
    }

    if input.is_empty() {
        let source = &valid_lines.join("\n");
        let raw_output = compiler(source);
        let output = raw_output.unwrap();

        print_output_repl(&output);

        return Ok(NextLoop::Continue);
    }

    valid_lines.push(input);

    let source = &valid_lines.join("\n");
    let raw_output = compiler(source);

    if let Err(error) = raw_output {
        report_repl_error(&error.to_string());

        valid_lines.pop();

        return Ok(NextLoop::Continue);
    }

    redo_lines.clear();

    let output = raw_output.unwrap();

    print_output_repl(&output);

    Ok(NextLoop::Continue)
}

fn repl_command(
    input: &str,
    valid_lines: &mut Vec<String>,
    redo_lines: &mut Vec<String>,
) -> NextLoop {
    match input {
        format_command!("u", "undo") => {
            if valid_lines.is_empty() {
                report_nothing_to_undo();
            } else {
                let latest = valid_lines.pop().unwrap();
                redo_lines.push(latest);

                if !valid_lines.is_empty() {
                    report_undo(false);
                    let source = &valid_lines.join("\n");
                    let raw_output = compiler(source);
                    let output = raw_output.unwrap();

                    print_output_repl(&output);
                } else {
                    report_undo(true);
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

            return NextLoop::Exit;
        }
        _ => report_unrecognized_command(input.trim()),
    }

    NextLoop::Continue
}

fn prompt_and_read() -> anyhow::Result<String> {
    prompt();
    Ok(read_input().map_err(|_| CliError::ReadInputError)?)
}
