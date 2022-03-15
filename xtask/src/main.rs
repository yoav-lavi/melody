use xshell::{cmd, Shell};

fn main() -> anyhow::Result<()> {
    let sh = Shell::new()?;
    sh.change_dir("crates/melody_wasm");
    cmd!(sh, "wasm-pack build --target web").run()?;
    cmd!(sh, "rm -r ../../playground/src/wasm").run()?;
    cmd!(sh, "cp -r ./pkg/. ../../playground/src/wasm").run()?;
    Ok(())
}
