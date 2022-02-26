# Changelog

## [v0.10.1] - 2022-02-26

### Fixes

- Trims only the end off of REPL input

## [v0.10.0] - 2022-02-26

### Breaking

- Changes the `-f, --file` CLI argument to `-o, --output`

### Features

- Adds descriptions to CLI commands

## [v0.9.0] - 2022-02-26

### Features

- Adds `ahead`, `not ahead`, `behind` and `not behind` assertions

## [v0.8.0] - 2022-02-26

### Features

- Changes `<space>` to `<whitespace>` (thanks @amirali #34)
- Adds `<space>` and `<alphabet>` (thanks @amirali #34)
- Adds long versions for REPL commands
- Adds `.s, .source` to print the current source in the REPL
- Adds `.c, .clear` to clear REPL history
- Adds better error reporting to the playground

### Fixes

- Fixes some undo / redo issues in the REPL

### Refactoring

- Better error handling in the CLI

## [v0.7.0] - 2022-02-24

### Features

- Adds a REPL for `melody_cli`
- Adds better error messages for the playground

## [v0.6.0] - 2022-02-23

### Features

- Adds support for raw sequences (<code>\`...\`</code>)
- Allows any word character in `capture` names
- Adds auto escaping for literals
- Adds the Melody version number to the documentation

### Syntax Changes

- Changes `start`, `end`, and `char` to symbols (`<start>`, `<end>`, `<char>`)
- `either` creates a non capturing group

### Refactoring

- `cargo clippy` fixes in `melody_wasm`

### Fixes

- Uses the correct `url` in the documentation site config
