use clap::Parser;

pub enum ExitCode {
    Ok,
    Error,
}

impl ExitCode {
    pub fn to_i32(&self) -> i32 {
        match self {
            ExitCode::Ok => 0,
            ExitCode::Error => 1,
        }
    }
}

#[derive(Parser, Debug)]
#[clap(about, version, author)]
pub struct Args {
    #[clap(value_name = "INPUT_FILE_PATH", help = "Read from a file")]
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
}

pub enum NextLoop {
    Continue,
    Exit,
}
