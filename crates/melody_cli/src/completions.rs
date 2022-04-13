use crate::output::report_unsupported_shell;
use crate::types::Args;
use clap::CommandFactory;
use clap_generate::generators::{generate, Bash, Elvish, Fish, PowerShell, Zsh};
use clap_generate::Generator;
use std::io;

pub fn generate_completions(shell: &str) {
    match shell {
        "bash" => completions_for_shell(Bash),
        "fish" => completions_for_shell(Fish),
        "zsh" => completions_for_shell(Zsh),
        "elvish" => completions_for_shell(Elvish),
        "powershell" => completions_for_shell(PowerShell),
        _ => report_unsupported_shell(shell),
    };
}

fn completions_for_shell(generator: impl Generator) {
    generate(generator, &mut Args::command(), "melody", &mut io::stdout());
}
