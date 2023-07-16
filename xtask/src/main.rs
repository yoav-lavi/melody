mod help;
mod utils;

use colored::control::ShouldColorize;
use help::Help;
use std::env;
use utils::{help_flag_used, positional_argument, shell_in_dir};
use xshell::{cmd, Shell};

fn main() {
    ShouldColorize::from_env();
    if let Err(error) = try_main() {
        eprintln!("{error}");
        std::process::exit(1);
    }
    std::process::exit(0);
}

fn try_main() -> anyhow::Result<()> {
    let task = positional_argument(1);

    if help_flag_used() {
        Help::general();
        return Ok(());
    }

    match task.as_deref() {
        Some("run") => run()?,
        Some("benchmark") => benchmark()?,
        Some("fuzz") => fuzz()?,
        Some("publish") => publish()?,
        Some("wasm") => wasm()?,
        _ => Help::Main.mistake(task.as_deref()),
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
        Some("extension") => publish_extension()?,
        Some("node") => publish_node()?,
        Some("playground") => publish_playground()?,
        _ => Help::Publish.mistake(target.as_deref()),
    }

    Ok(())
}

fn fuzz() -> anyhow::Result<()> {
    let target = positional_argument(2);

    match target.as_deref() {
        Some("compiler") => fuzz_compiler()?,
        _ => Help::Fuzz.mistake(target.as_deref()),
    }

    Ok(())
}

fn publish_extension() -> anyhow::Result<()> {
    let extension = positional_argument(3);

    match extension.as_deref() {
        Some("vscode") => publish_extension_vscode()?,
        _ => Help::PublishExtension.mistake(extension.as_deref()),
    }

    Ok(())
}

fn wasm() -> anyhow::Result<()> {
    let target = positional_argument(2);

    match target.as_deref() {
        Some("playground") => wasm_playground()?,
        Some("node") => wasm_node()?,
        _ => Help::Wasm.mistake(target.as_deref()),
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
    let shell = shell_in_dir("crates/melody_compiler")?;

    cmd!(shell, "cargo publish").run()?;

    Ok(())
}

fn publish_extension_vscode() -> anyhow::Result<()> {
    let shell = shell_in_dir("extensions/vscode")?;

    cmd!(shell, "vsce publish").run()?;

    Ok(())
}

fn publish_node() -> anyhow::Result<()> {
    let shell = shell_in_dir("crates/melody_wasm")?;

    // to prevent cases where we publish
    // different bindings
    wasm_node()?;

    shell.change_dir("./pkg");

    cmd!(shell, "npm publish --access=public").run()?;

    Ok(())
}

fn publish_playground() -> anyhow::Result<()> {
    let shell = shell_in_dir("playground")?;

    // to prevent cases where we publish
    // different bindings
    wasm_playground()?;

    cmd!(shell, "vercel").run()?;
    cmd!(shell, "vercel --prod").run()?;

    Ok(())
}

fn wasm_playground() -> anyhow::Result<()> {
    let shell = shell_in_dir("crates/melody_wasm")?;

    cmd!(shell, "wasm-pack build --target web").run()?;
    cmd!(shell, "rm -r -f ../../playground/wasm").run()?;
    cmd!(shell, "cp -r ./pkg/. ../../playground/wasm").run()?;

    Ok(())
}

fn wasm_node() -> anyhow::Result<()> {
    let shell = shell_in_dir("crates/melody_wasm")?;

    cmd!(shell, "rm -r -f ./pkg/").run()?;
    cmd!(shell, "wasm-pack build --target nodejs").run()?;
    cmd!(
        shell,
        "sed -i '' 's/\"name\":.*/\"name\": \"melodyc\",/g' ./pkg/package.json"
    )
    .run()?;
    cmd!(
        shell,
        "sed -i '' 's/\"description\":.*/\"description\": \"NodeJS bindings for the Melody language compiler\",/g' ./pkg/package.json"
    )
    .run()?;

    cmd!(shell, "rm ./pkg/README.md").run()?;

    cmd!(shell, "cp ../../xtask/assets/node-readme.md ./pkg/README.md").run()?;

    Ok(())
}
