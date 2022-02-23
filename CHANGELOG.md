# Changelog

## [v0.6.0] - 2022-02-23

### Features

- Adds support for raw sequences (<code>\`...\`</code>)
- Allows any word character in `capture` names
- Adds auto escaping for literals
- Adds the Melody version number to the documentation

### Syntax Changes

- Changes `start`, `end`, and `char` to symbols (`<start>`, `<end>`, `<char>`)

### Refactoring

- `cargo clippy` fixes in `melody_wasm`

### Fixes

- Uses the correct `url` in the documentation site config
