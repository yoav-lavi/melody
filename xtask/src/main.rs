use std::env;

use xshell::{cmd, Shell};

fn main() {
    if let Err(e) = try_main() {
        eprintln!("{}", e);
        std::process::exit(-1);
    }
}

fn try_main() -> anyhow::Result<()> {
    let task = env::args().nth(1);
    match task.as_ref().map(|task| task.as_str()) {
        Some("wasm") => wasm()?,
        Some("deploy-playground") => deploy_playground()?,
        Some("deploy-docs") => deploy_docs()?,
        Some("publish-compiler") => publish_compiler()?,
        Some("publish-cli") => publish_cli()?,
        _ => print_help(),
    }
    Ok(())
}

fn wasm() -> anyhow::Result<()> {
    let sh = Shell::new()?;
    sh.change_dir("crates/melody_wasm");
    cmd!(sh, "wasm-pack build --target web").run()?;
    cmd!(sh, "rm -r ../../playground/src/wasm").run()?;
    cmd!(sh, "cp -r ./pkg/. ../../playground/src/wasm").run()?;
    Ok(())
}

fn deploy_playground() -> anyhow::Result<()> {
    let sh = Shell::new()?;
    sh.change_dir("playground");
    cmd!(sh, "vercel").run()?;
    cmd!(sh, "vercel --prod").run()?;
    Ok(())
}

fn deploy_docs() -> anyhow::Result<()> {
    let sh = Shell::new()?;
    sh.change_dir("docs");
    cmd!(sh, "yarn deploy").run()?;
    Ok(())
}

fn publish_cli() -> anyhow::Result<()> {
    let sh = Shell::new()?;
    sh.change_dir("crates/melody_cli");
    cmd!(sh, "cargo publish").run()?;
    Ok(())
}

fn publish_compiler() -> anyhow::Result<()> {
    let sh = Shell::new()?;
    sh.change_dir("crates/melody_cli");
    cmd!(sh, "cargo publish").run()?;
    Ok(())
}

fn print_help() {
    eprintln!(
        r#"Tasks: 
    wasm
    deploy-playground
    deploy-docs
    publish-compiler
    publish-cli
    "#
    )
}
