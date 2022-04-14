use atty::Stream;
use clap::Parser;

#[derive(Parser, Debug)]
#[clap(about, version, author)]
pub struct Args {
    #[clap(
        value_name = "INPUT_FILE_PATH",
        help = "Read from a file\nUse '-' and or pipe input to read from stdin"
    )]
    pub input_file_path: Option<String>,
    #[clap(
        short = 'o',
        long = "output",
        value_name = "OUTPUT_FILE_PATH",
        help = "Write to a file"
    )]
    pub output_file_path: Option<String>,
    #[clap(short = 'n', long = "no-color", help = "Print output with no color")]
    pub no_color_output: bool,
    #[clap(
        short = 'c',
        long = "clean",
        help = "Print output without opening and closing slashes, flags or newlines. Does not affect the REPL"
    )]
    #[clap(short = 'r', long = "repl", help = "Start the Melody REPL")]
    pub start_repl: bool,
    #[clap(
        long = "generate-completions",
        help = "Outputs completions for the selected shell\nTo use, write the output to the appropriate location for your shell",
        conflicts_with_all = &["output-file-path", "input-file-path", "start-repl"]
    )]
    pub completions: Option<String>,
    #[clap(
        long = "test",
        short = 't',
        help = "Test the compiled regex against a string",
        conflicts_with_all = &["completions", "start-repl", "output-file-path"]
    )]
    pub test: Option<String>,
    #[clap(
        long = "test-file",
        short = 'f',
        help = "Test the compiled regex against the contents of a file",
        conflicts_with_all = &["completions", "start-repl", "output-file-path", "test"]
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
            stdin: !atty::is(Stream::Stdin),
            stdout: !atty::is(Stream::Stdout),
            // stderr: !atty::is(Stream::Stderr),
        }
    }
    pub fn any_pipe(&self) -> bool {
        self.stdin || self.stdout
    }
}
