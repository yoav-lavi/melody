<p align="center">
    <img alt="Melody Logo" height="250px" src="https://user-images.githubusercontent.com/14347895/159069181-53bce5b3-a831-43f1-8c14-af6c6ed7b92b.svg">
</p>

<p align="center">
A CLI wrapping the Melody language compiler
</p>

## Install

```sh
cargo install melody_cli
```

## Usage

```sh
USAGE:
    melody [OPTIONS] [INPUT_FILE_PATH]

ARGS:
    <INPUT_FILE_PATH>    Read from a file
                         Use '-' and or pipe input to read from stdin

OPTIONS:
        --generate-completions <COMPLETIONS>
            Outputs completions for the selected shell
            To use, write the output to the appropriate location for your shell

    -h, --help
            Print help information

    -n, --no-color
            Print output with no color

    -o, --output <OUTPUT_FILE_PATH>
            Write to a file

    -r, --repl
            Start the Melody REPL

    -t, --test <TEST>
            Test the compiled regex against a string

    -V, --version
            Print version information
```
