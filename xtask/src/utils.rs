use std::env;
use xshell::Shell;

pub fn shell_in_dir(dir: &str) -> anyhow::Result<Shell> {
    let shell = Shell::new()?;
    shell.change_dir(dir);
    Ok(shell)
}

pub fn positional_argument(n: usize) -> Option<String> {
    env::args().nth(n)
}
