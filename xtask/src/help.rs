use colored::{self, Colorize};
use std::fmt::Display;

pub enum Help {
    Main,
    Fuzz,
    Publish,
    PublishExtension,
    Wasm,
}

fn print_suggestion(current: Option<&str>, for_command: &str) {
    match current {
        Some(current) => eprintln!(
            "Did you mean this?\n\t{} {}\n",
            current.bright_blue(),
            for_command.bright_blue()
        ),
        None => eprintln!("Did you mean this?\n\t{}\n", for_command.bright_blue()),
    }
}

fn print_not_a_command(current: Option<&str>, command: &str) {
    match current {
        Some(current) => eprintln!("'{current} {command}' is not a command.\n"),
        None => eprintln!("'{command}' is not a command.\n"),
    }
}

fn print_for_more_information() {
    eprintln!("For more information, see 'cargo xtask --help'.");
}

impl Help {
    pub fn general() {
        println!("usage: cargo xtask <command>\n");
        println!("A CLI allowing to run various tasks in the Melody repository\n");
        println!("{}\n", Self::Main);
        println!("{}\n", Self::Fuzz);
        println!("{}\n", Self::Publish);
        println!("{}\n", Self::PublishExtension);
        println!("{}", Self::Wasm);
    }

    pub fn mistake(&self, command: Option<&str>) {
        if let Some(command) = command {
            self.unrecognized_command(command);
        }

        eprintln!("{self}");

        if command.is_some() {
            print_for_more_information();
        }
    }

    fn unrecognized_command(&self, command: &str) {
        match self {
            Self::Main => {
                print_not_a_command(None, command);
                match command {
                    "deploy" => print_suggestion(None, "publish"),
                    "bench" => print_suggestion(None, "benchmark"),
                    "fuzzer" => print_suggestion(None, "fuzz"),
                    "WASM" => print_suggestion(None, "wasm"),
                    _ => {}
                }
            }
            Self::Publish => {
                print_not_a_command(Some("publish"), command);
                match command {
                    "nodejs" => print_suggestion(Some("publish"), "node"),
                    "vs-code" | "vscode" => print_suggestion(Some("publish"), "extension vscode"),
                    "CLI" => print_suggestion(Some("publish"), "cli"),
                    _ => {}
                }
            }
            Self::Wasm => {
                print_not_a_command(Some("wasm"), command);
                #[allow(clippy::single_match)]
                match command {
                    "nodejs" => print_suggestion(Some("wasm"), "node"),
                    _ => {}
                }
            }
            Self::PublishExtension => {
                print_not_a_command(Some("publish extension"), command);
                #[allow(clippy::single_match)]
                match command {
                    "vs-code" => print_suggestion(Some("publish extension"), "vscode"),
                    _ => {}
                }
            }
            Self::Fuzz => print_not_a_command(Some("fuzz"), command),
        }
    }
}

impl Display for Help {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Main => formatter.write_str(
                r#"commands:
      run [arguments]          runs the melody_cli binary
      benchmark                runs benchmarks
      fuzz <crate>             runs fuzz testing on specific crates
      publish <target>         publishes specific crates or projects
      wasm <target>            builds wasm dependencies for specific projects
      "#,
            ),
            Self::Fuzz => formatter.write_str(
                r#"fuzz subcommands:
      compiler                 runs fuzz testing on the compiler
      "#,
            ),
            Self::Publish => formatter.write_str(
                r#"publish subcommands:
      cli                      publishes the melody_cli crate to crates.io
      compiler                 publishes the melody_compiler crate to crates.io
      playground               publishes the playground to vercel
      node                     publishes the melodyc npm package
      extension <target>       publishes specific extensions
      "#,
            ),
            Self::PublishExtension => formatter.write_str(
                r#"publish extension subcommands:
      vscode                   publishes the vscode extension to the market
      "#,
            ),
            Self::Wasm => formatter.write_str(
                r#"wasm subcommands:
      playground               builds wasm dependencies for the playground
      node                     builds wasm dependencies for nodejs
      "#,
            ),
        }
    }
}
