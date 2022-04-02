use std::fmt::Display;

pub enum Help {
    Main,
    Fuzz,
    Publish,
    PublishExtension,
    Wasm,
}

impl Help {
    pub fn print(&self) {
        eprintln!("{}", self)
    }
}

impl Display for Help {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Help::Main => formatter.write_str(
                r#"commands:
      run - runs the melody cli binary
      benchmark - runs benchmarks
      fuzz - runs fuzz testing on specific crates
      publish - publishes specific crates or projects
      wasm - builds wasm dependencies for specific projects
      "#,
            ),
            Help::Fuzz => formatter.write_str(
                r#"fuzz commands:
      compiler - runs fuzz testing on the compiler
      "#,
            ),
            Help::Publish => formatter.write_str(
                r#"publish commands:
      cli - publishes the cli to crates.io
      compiler - publishes the compiler to crates.io
      docs - publishes the docs to github pages
      extension - publishes specific extensions
      playground - publishes the playground to vercel
      node - publishes the melodyc npm package
      "#,
            ),
            Help::PublishExtension => formatter.write_str(
                r#"publish extension commands:
      vscode - publishes the vscode extension to the market
      "#,
            ),
            Help::Wasm => formatter.write_str(
                r#"wasm commands:
      playground - builds wasm dependencies for the playground
      node - builds wasm dependencies for nodejs
      "#,
            ),
        }
    }
}
