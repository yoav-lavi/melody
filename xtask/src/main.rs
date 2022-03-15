use std::env;
use xshell::{cmd, Shell};

fn main() {
    if let Err(error) = try_main() {
        eprintln!("{error}");
        std::process::exit(1);
    }
}

fn try_main() -> anyhow::Result<()> {
    let task = env::args().nth(1);
    match task.as_ref().map(|task| task.as_str()) {
        Some("deploy-docs") => deploy_docs()?,
        Some("deploy-playground") => deploy_playground()?,
        Some("playground-wasm") => playground_wasm()?,
        Some("publish-cli") => publish_cli()?,
        Some("publish-compiler") => publish_compiler()?,
        _ => print_help(),
    }
    Ok(())
}

fn deploy_docs() -> anyhow::Result<()> {
    let shell = Shell::new()?;
    shell.change_dir("docs");
    cmd!(shell, "yarn deploy").run()?;
    Ok(())
}

fn deploy_playground() -> anyhow::Result<()> {
    let shell = Shell::new()?;
    shell.change_dir("playground");
    cmd!(shell, "vercel").run()?;
    cmd!(shell, "vercel --prod").run()?;
    Ok(())
}

fn playground_wasm() -> anyhow::Result<()> {
    let shell = Shell::new()?;
    shell.change_dir("crates/melody_wasm");
    cmd!(shell, "wasm-pack build --target web").run()?;
    cmd!(shell, "rm -r ../../playground/src/wasm").run()?;
    cmd!(shell, "cp -r ./pkg/. ../../playground/src/wasm").run()?;
    Ok(())
}

fn publish_cli() -> anyhow::Result<()> {
    let shell = Shell::new()?;
    shell.change_dir("crates/melody_cli");
    cmd!(shell, "cargo publish").run()?;
    Ok(())
}

fn publish_compiler() -> anyhow::Result<()> {
    let shell = Shell::new()?;
    shell.change_dir("crates/melody_cli");
    cmd!(shell, "cargo publish").run()?;
    Ok(())
}

fn print_help() {
    eprintln!(
        r#"Tasks:
    deploy-docs
    deploy-playground
    playground-wasm
    publish-cli
    publish-compiler
    "#
    )
}
