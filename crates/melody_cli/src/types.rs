use clap::Parser;
use std::io::IsTerminal;

#[derive(Parser, Debug)]
#[clap(about, version, author)]
pub struct Args {
    #[clap(
        id = "input",
        value_name = "INPUT_FILE_PATH",
        help = "Read from a file\nUse '-' and or pipe input to read from stdin"
    )]
    pub input_file_path: Option<String>,
    #[clap(
        id = "output",
        short = 'o',
        long = "output",
        value_name = "OUTPUT_FILE_PATH",
        help = "Write to a file"
    )]
    pub output_file_path: Option<String>,
    #[clap(id = "no-color", short = 'n', long = "no-color", help = "Print output with no color")]
    pub no_color_output: bool,
    #[clap(id = "repl", short = 'r', long = "repl", help = "Start the Melody REPL")]
    pub start_repl: bool,
    #[clap(
        id = "completions",
        long = "generate-completions",
        help = "Outputs completions for the selected shell\nTo use, write the output to the appropriate location for your shell",
        conflicts_with_all = &["output", "input", "repl"]
    )]
    pub completions: Option<String>,
    #[clap(
        id = "test",
        long = "test",
        short = 't',
        help = "Test the compiled regex against a string",
        conflicts_with_all = &["completions", "repl", "output"]
    )]
    pub test: Option<String>,
    #[clap(
        id = "test-file",
        long = "test-file",
        short = 'f',
        help = "Test the compiled regex against the contents of a file",
        conflicts_with_all = &["completions", "repl", "output", "test"]
    )]
    pub test_file: Option<String>,
}

pub enum NextLoop {
    Continue,
    Exit,
}

pub struct Streams {
    pub stdin: bool,
    pub stdout: bool,
    // pub stderr: bool,
}

impl Streams {
    pub fn new() -> Self {
        Self {
            stdin: !std::io::stdin().is_terminal(),
            stdout: !std::io::stdout().is_terminal(),
            // stderr: !std::io::stderr().is_terminal(),
        }
    }
    pub const fn any_pipe(&self) -> bool {
        self.stdin || self.stdout
    }
}

#[test]
fn verify_cli() {
    use clap::CommandFactory;
    Args::command().debug_assert();
}
