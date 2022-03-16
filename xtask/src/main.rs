mod help;
mod utils;

use std::env;

use help::Help;
use utils::{positional_argument, shell_in_dir};
use xshell::{cmd, Shell};

fn main() {
    if let Err(error) = try_main() {
        eprintln!("{error}");
        std::process::exit(1);
    }
}

fn try_main() -> anyhow::Result<()> {
    let task = positional_argument(1);

    match task.as_deref() {
        Some("run") => run()?,
        Some("benchmark") => benchmark()?,
        Some("fuzz") => fuzz()?,
        Some("publish") => publish()?,
        Some("wasm") => wasm()?,
        _ => Help::Main.print(),
    }

    Ok(())
}

fn run() -> anyhow::Result<()> {
    let shell = Shell::new()?;

    let arguments: Vec<String> = env::args().skip(2).collect();

    cmd!(shell, "cargo run --bin melody {arguments...}").run()?;

    Ok(())
}

fn benchmark() -> anyhow::Result<()> {
    let shell = Shell::new()?;

    cmd!(shell, "cargo criterion --output-format=verbose").run()?;

    Ok(())
}

fn publish() -> anyhow::Result<()> {
    let target = positional_argument(2);

    match target.as_deref() {
        Some("cli") => publish_cli()?,
        Some("compiler") => publish_compiler()?,
        Some("docs") => publish_docs()?,
        Some("extension") => publish_extension()?,
        Some("playground") => publish_playground()?,
        _ => Help::Publish.print(),
    }

    Ok(())
}

fn fuzz() -> anyhow::Result<()> {
    let target = positional_argument(2);

    match target.as_deref() {
        Some("compiler") => fuzz_compiler()?,
        _ => Help::Fuzz.print(),
    }

    Ok(())
}

fn publish_extension() -> anyhow::Result<()> {
    let extension = positional_argument(3);

    match extension.as_deref() {
        Some("vscode") => publish_extension_vscode()?,
        _ => Help::PublishExtension.print(),
    }

    Ok(())
}

fn wasm() -> anyhow::Result<()> {
    let target = positional_argument(2);

    match target.as_deref() {
        Some("playground") => wasm_playground()?,
        _ => Help::Wasm.print(),
    }

    Ok(())
}

fn fuzz_compiler() -> anyhow::Result<()> {
    let shell = shell_in_dir("crates/melody_compiler/fuzz")?;

    cmd!(shell, "cargo +nightly fuzz run ast_fuzz").run()?;

    Ok(())
}

fn publish_cli() -> anyhow::Result<()> {
    let shell = shell_in_dir("crates/melody_cli")?;

    cmd!(shell, "cargo publish").run()?;

    Ok(())
}

fn publish_compiler() -> anyhow::Result<()> {
    let shell = shell_in_dir("crates/melody_cli")?;

    cmd!(shell, "cargo publish").run()?;

    Ok(())
}

fn publish_docs() -> anyhow::Result<()> {
    let shell = shell_in_dir("docs")?;

    cmd!(shell, "yarn deploy").run()?;

    Ok(())
}

fn publish_extension_vscode() -> anyhow::Result<()> {
    let shell = shell_in_dir("extensions/vscode")?;

    cmd!(shell, "vsce publish").run()?;

    Ok(())
}

fn publish_playground() -> anyhow::Result<()> {
    let shell = shell_in_dir("playground")?;

    cmd!(shell, "vercel").run()?;
    cmd!(shell, "vercel --prod").run()?;

    Ok(())
}

fn wasm_playground() -> anyhow::Result<()> {
    let shell = shell_in_dir("crates/melody_wasm")?;

    cmd!(shell, "wasm-pack build --target web").run()?;
    cmd!(shell, "rm -r ../../playground/wasm").run()?;
    cmd!(shell, "cp -r ./pkg/. ../../playground/wasm").run()?;

    Ok(())
}
