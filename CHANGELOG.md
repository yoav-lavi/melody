# Changelog

## [v0.19.0] - 2023-07-16

### Breaking

- Sets the MSRV to Rust 1.65.0

### Features

- Adds error output for panics

### Dependencies

- Updates dependencies

### Refactoring

- Clippy fixes

## [v0.18.1] - 2022-06-25

### Fixes

- Fixes playground link

### Dependencies

- Updates dependencies

### Refactoring

- Clippy fixes

## [v0.18.0] - 2022-04-24

### Features

- Adds support for unicode categories

### Misc.

- Update dependencies

## [v0.17.0] - 2022-04-23

### Features

- Add support for testing matches in a file in the CLI

### Refactoring

- Remove `anyhow` in compiler in favor of emitting specific error variants

## [v0.16.0] - 2022-04-13

### Features

- Add support for testing matches in CLI

## [v0.15.0] - 2022-04-13

### Features

- Add shell completions for CLI
- Add Deno support

## [v0.14.0] - 2022-04-11

### Features

- Support stdin in CLI
- Emit proper exit codes on specific errors

## [v0.13.10] - 2022-03-11

### Fixes

- Fixes unnecessary grouping in quantifiers

## [v0.13.9] - 2022-03-11

### Misc.

- Version bump for documentation update

## [v0.13.8] - 2022-03-11

### Misc.

- Version bump for documentation update

## [v0.13.7] - 2022-03-11

### Misc.

- Version bump for documentation update

## [v0.13.6] - 2022-03-11

### Fixes

- Handles a few possible panics

## [v0.13.5] - 2022-03-11

### Misc.

- Version bump

## [v0.13.4] - 2022-03-11

### Tooling

- Strips binaries

### Dependencies

- Updates dependencies

## [v0.13.3] - 2022-03-09

### Refactoring

- Replaces `lazy_static` with `once_cell`

## [v0.13.2] - 2022-03-09

### Performance

- Improves literal parse performance

### Refactoring

- Reports a few possible panics with a ParseError

## [v0.13.1] - 2022-03-08

### Fixes

- Fixes an issue with single letter variable identifiers matching a following space
- Fixes a clash between REPL commands and variables

## [v0.13.0] - 2022-03-08

### Breaking

- `<alphabet>` is now `<alphabetic>`

### Features

- Support for lazy quantifiers
- All symbols now have negative counterparts
- `<alphanumeric>` symbol added
- Adds an experimental implementation of variables

## [v0.12.4] - 2022-03-06

### Misc.

- Version bump

## [v0.12.3] - 2022-03-06

### Fixes

- Fixes an issue with identifying negative char ranges

## [v0.12.2] - 2022-03-05

### Refactoring

- Performance improvements

### Misc.

- Adds keywords and categories to cargo.toml files

## [v0.12.1] - 2022-03-04

### Misc.

- CLI documentation update

## [v0.12.0] - 2022-03-04

### Breaking

- Produces clean output (no `//` and new newline after output)

### Features

- Adds favicons for documentation and playground
- The Melody playground now supports add to homescreen
- Adds `#![forbid(unsafe_code)]`

### Benchmarks

- Adds benchmarks

## [v0.11.1] - 2022-03-03

### Fixes

- Fixes possible panics

### Tests

- Adds tests
- Adds tests for CLI

### Refactoring

- Removes duplicated code

## [v0.11.0] - 2022-03-02

### Breaking

- `ParseError` now contains only one `message` field, may be changed in the future
- Line comments (`//`) may only be used in a separate line
- The REPL currently accepts blocks on a single line but not multiple lines
- Semicolons are no longer optional

### Features

- Uses a Pest grammar and an AST to parse Melody
- Adds support for nested groups
- Adds support for negative ranges
- Adds initial support for negative character classes
- Adds support for `<backspace>`, `<boundary>`
- Adds support for inline comments
- Enforces group closing
- Supports NO_COLOR in CLI
- `-n` removes color from REPL as well

## [v0.10.3] - 2022-02-26

### Fixes

- Removes quantifiers after newlines

## [v0.10.2] - 2022-02-26

### Fixes

- Fixes the handling of some newline issues in the REPL
- Adds an error message for a read error in the REPL

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
