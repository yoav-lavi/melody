<p align="center">
    <img alt="Melody Logo" height="250px" src="https://user-images.githubusercontent.com/14347895/159065887-51a2d948-ae6f-48c4-9dd2-1ee69e76b19f.png">
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
melody [OPTIONS] [INPUT_FILE_PATH]

ARGS:
    <INPUT_FILE_PATH>    Read from a file

OPTIONS:
    -h, --help                         Print help information
    -n, --no-color                     Print output with no color
    -o, --output <OUTPUT_FILE_PATH>    Write to a file
    -r, --repl                         Start the Melody REPL
    -V, --version                      Print version information
```