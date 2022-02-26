use std::io::Write;

pub fn read_input() -> String {
    let _ = std::io::stdout().flush();
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    String::from(input.trim_end())
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
