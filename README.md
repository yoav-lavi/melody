<p align="center">
  <img alt="Melody Logo" height="250px" src="https://user-images.githubusercontent.com/14347895/157920860-2dfc3278-1526-4a96-a3d3-97b5daca8390.png#gh-dark-mode-only">
    <img alt="Melody Logo" height="250px" src="https://user-images.githubusercontent.com/14347895/157926614-8434c590-e810-494c-ac9d-3657e9aa4583.png#gh-light-mode-only">
</p>

<p align="center">
  <a href="https://github.com/yoav-lavi/melody/actions/workflows/rust.yml">
    <img alt="Rust CI" src="https://github.com/yoav-lavi/melody/actions/workflows/rust.yml/badge.svg">
  </a>
  <a href="https://crates.io/crates/melody_compiler">
    <img alt="Crates.io" src="https://img.shields.io/crates/v/melody_compiler?label=compiler">
  </a>
  <a href="https://crates.io/crates/melody_cli">
    <img alt="Crates.io" src="https://img.shields.io/crates/v/melody_cli?label=cli">
  </a>
  <a href="https://github.com/yoav-lavi/melody/blob/main/LICENSE">
    <img alt="Crates.io" src="https://img.shields.io/crates/l/melody_compiler">
  </a>
  <a href="https://melody-playground.vercel.app">
    <img alt="melody playground" src="https://img.shields.io/badge/melody-playground-brightgreen">
  </a>
  <a href="https://yoav-lavi.github.io/melody/docs/intro">
    <img alt="melody playground" src="https://img.shields.io/badge/melody-docs-blue">
  </a>
</p>

<p align="center">
Melody is a language that compiles to ECMAScript regular expressions, while aiming to be more readable and maintainable.
</p>

<p align="center">
  <img width="400" alt="code example" src="https://user-images.githubusercontent.com/14347895/154124756-ddbd3c84-f8b2-45bd-b624-2c510482c4e2.png">
</p>


## Examples


Note: these are for the currently supported syntax and may change

### Batman Theme &nbsp;<sub><sup><a href="https://melody-playground.vercel.app?content=MTYlMjBvZiUyMCUyMm5hJTIyJTNCJTBBJTBBMiUyMG9mJTIwbWF0Y2glMjAlN0IlMEElMjAlMjAlM0NzcGFjZSUzRSUzQiUwQSUyMCUyMCUyMmJhdG1hbiUyMiUzQiUwQSU3RCUwQSUwQSUyRiUyRiUyMCVGMCU5RiVBNiU4NyVGMCU5RiVBNiVCOCVFMiU4MCU4RCVFMiU5OSU4MiVFRiVCOCU4Rg==">try in playground</a></sup></sub>

```rust
16 of "na";

2 of match {
  <space>;
  "batman";
}

// 🦇🦸‍♂️
```

Turns into

```regex
(?:na){16}(?: batman){2}
```

### Twitter Hashtag &nbsp;<sub><sup><a href="https://melody-playground.vercel.app?content=JTIyJTIzJTIyJTNCJTBBc29tZSUyMG9mJTIwJTNDd29yZCUzRSUzQiUwQSUwQSUyRiUyRiUyMCUyM21lbG9keQ==">try in playground</a></sup></sub>

```rust
"#";
some of <word>;

// #melody
```

Turns into

```regex
#(?:\w)+
```

### Introductory Courses &nbsp;<sub><sup><a href="https://melody-playground.vercel.app?content=c29tZSUyMG9mJTIwJTNDYWxwaGFiZXRpYyUzRSUzQiUwQSUzQ3NwYWNlJTNFJTNCJTBBJTIyMSUyMiUzQiUwQTIlMjBvZiUyMCUzQ2RpZ2l0JTNFJTNCJTBBJTBBJTJGJTJGJTIwY2xhc3NuYW1lJTIwMXh4">try in playground</a></sup></sub>

```rust
some of <alphabetic>;
<space>;
"1";
2 of <digit>;

// classname 1xx
```

Turns into

```regex
(?:[a-zA-Z])+ 1(?:\d){2}
```

### Indented Code (2 spaces) &nbsp;<sub><sup><a href="https://melody-playground.vercel.app?content=c29tZSUyMG9mJTIwbWF0Y2glMjAlN0IlMEElMjAlMjAyJTIwb2YlMjAlM0NzcGFjZSUzRSUzQiUwQSU3RCUwQSUwQXNvbWUlMjBvZiUyMCUzQ2NoYXIlM0UlM0IlMEElMjIlM0IlMjIlM0IlMEElMEElMkYlMkYlMjBsZXQlMjB2YWx1ZSUyMCUzRCUyMDUlM0I=">try in playground</a></sup></sub>

```rust
some of match {
  2 of <space>;
}

some of <char>;
";";

// let value = 5;
```

Turns into

```regex
(?: {2})+.+;
```

### Semantic Versions &nbsp;<sub><sup><a href="https://melody-playground.vercel.app?content=JTNDc3RhcnQlM0UlM0IlMEElMEFvcHRpb24lMjBvZiUyMCUyMnYlMjIlM0IlMEElMEFjYXB0dXJlJTIwbWFqb3IlMjAlN0IlMEElMjAlMjBzb21lJTIwb2YlMjAlM0NkaWdpdCUzRSUzQiUwQSU3RCUwQSUwQSUyMi4lMjIlM0IlMEElMEFjYXB0dXJlJTIwbWlub3IlMjAlN0IlMEElMjAlMjBzb21lJTIwb2YlMjAlM0NkaWdpdCUzRSUzQiUwQSU3RCUwQSUwQSUyMi4lMjIlM0IlMEElMEFjYXB0dXJlJTIwcGF0Y2glMjAlN0IlMEElMjAlMjBzb21lJTIwb2YlMjAlM0NkaWdpdCUzRSUzQiUwQSU3RCUwQSUwQSUzQ2VuZCUzRSUzQiUwQSUwQSUyRiUyRiUyMHYxLjAuMA==">try in playground</a></sup></sub>

```rust
<start>;

option of "v";

capture major {
  some of <digit>;
}

".";

capture minor {
  some of <digit>;
}

".";

capture patch {
  some of <digit>;
}

<end>;

// v1.0.0
```

Turns into

```regex
^v?(?<major>(?:\d)+)\.(?<minor>(?:\d)+)\.(?<patch>(?:\d)+)$
```

## Playground

You can try Melody in your browser using the [playground](https://melody-playground.vercel.app/)

## Documentation

Read the documentation [here](https://yoav-lavi.github.io/melody/)

## Install

### Cargo

```sh
cargo install melody_cli
```

### From Source

```sh
git clone https://github.com/yoav-lavi/melody.git
cd melody
cargo install --path crates/melody_cli
```
### Binary

- macOS binaries (aarch64 and x86_64) can be downloaded from the [release page](https://github.com/yoav-lavi/melody/releases)

### Community

- [Arch Linux](https://aur.archlinux.org/packages/melody) (maintained by [@ilai-deutel](https://github.com/ilai-deutel))
  <details><summary>Installation instructions</summary>

  1. Installation with an AUR helper, for instance using `paru`:

     ```bash
     paru -Syu melody
     ```

  2. Install manually with `makepkg`:

     ```bash
     git clone https://aur.archlinux.org/melody.git
     cd melody
     makepkg -si
     ```

  </details>

- NixOS (soon, see [PR](https://github.com/NixOS/nixpkgs/pull/160985)) (maintained by [@jyooru](https://github.com/jyooru))

## CLI Usage

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

## Changelog

See the changelog [here](https://github.com/yoav-lavi/melody/blob/main/CHANGELOG.md) or in the [release page](https://github.com/yoav-lavi/melody/releases)

## Syntax

### Quantifiers

- `... of` - used to express a specific amount of a pattern. equivalent to regex `{5}` (assuming `5 of ...`)
- `... to ... of` - used to express an amount within a range of a pattern. equivalent to regex `{5,9}` (assuming `5 to 9 of ...`)
- `over ... of` - used to express more than an amount of a pattern. equivalent to regex `{6,}` (assuming `over 5 of ...`)
- `some of` - used to express 1 or more of a pattern. equivalent to regex `+`
- `any of` - used to express 0 or more of a pattern. equivalent to regex `*`
- `option of` - used to express 0 or 1 of a pattern. equivalent to regex `?`

All quantifiers can be preceded by `lazy` to match the least amount of characters rather than the most characters (greedy). Equivalent to regex `+?`, `*?`, etc.

### Symbols

- `<char>` - matches any single character. equivalent to regex `.`
- `<whitespace>` - matches any kind of whitespace character. equivalent to regex `\s` or `[ \t\n\v\f\r]`
- `<newline>` - matches a newline character. equivalent to regex `\n`  or `[0-9]`
- `<tab>` - matches a tab character. equivalent to regex `\t`
- `<return>` -  matches a carriage return character. equivalent to regex `\r`
- `<feed>` - matches a form feed character. equivalent to regex `\f`
- `<null>` - matches a null characther. equivalent to regex `\0`
- `<digit>` - matches any single digit. equivalent to regex `\d` or `[0-9]`
- `<vertical>` - matches a vertical tab character. equivalent to regex `\v`
- `<word>` - matches a word character (any latin letter, any digit or an underscore). equivalent to regex `\w` or `[a-zA-Z0-9_]`
- `<alphabetic>` - matches any single latin letter. equivalent to regex `[a-zA-Z]`
- `<alphanumeric>` - matches any single latin letter or any single digit. equivalent to regex `[a-zA-Z0-9]`
- `<boundary>` - Matches a character between a character matched by `<word>` and a character not matched by `<word>` without consuming the character. equivalent to regex `\b`
- `<backspace>` - matches a backspace control character. equivalent to regex `[\b]`

All symbols can be preceeded with `not` to match any character other than the symbol

### Special Symbols

- `<start>` - matches the start of the string. equivalent to regex `^`
- `<end>` - matches the end of the string. equivalent to regex `$`

### Character Ranges

- `... to ...` - used with digits or alphabetic characters to express a character range. equivalent to regex `[5-9]` (assuming `5 to 9`) or `[a-z]` (assuming `a to z`)

### Literals

- `"..."` or `'...'` - used to mark a literal part of the match. Melody will automatically escape characters as needed. Quotes (of the same kind surrounding the literal) should be escaped

### Raw

- <code>\`...\`</code> - added directly to the output without any escaping

### Groups

- `capture` - used to open a `capture` or named `capture` block. capture patterns are later available in the list of matches (either positional or named). equivalent to regex `(...)`
- `match` - used to open a `match` block, matches the contents without capturing. equivalent to regex `(?:...)`
- `either` - used to open an `either` block, matches one of the statements within the block. equivalent to regex `(?:...|...)`

### Assertions

- `ahead` - used to open an `ahead` block. equivalent to regex `(?=...)`. use after an expression
- `behind` - used to open an `behind` block. equivalent to regex `(?<=...)`. use before an expression

Assertions can be preceeded by `not` to create a negative assertion (equivalent to regex `(?!...)`, `(?<!...)`)

### Variables

- `let .variable_name = { ... }` - defines a variable from a block of statements. can later be used with `.variable_name`. Variables must be declared before being used. Variable invocations cannot be quantified directly, use a group if you want to quantify a variable invocation

  example:

  ```rs
  let .a_and_b = {
    "a";
    "b";
  }

  .a_and_b;
  "c";

  // abc

### Extras

- `/* ... */`, `// ...` - used to mark comments (note: `// ...` comments must be on separate line)

## File Extension

The Melody file extension is `.mdy`

## Crates

- `melody_compiler` - The Melody compiler [📦](https://crates.io/crates/melody_compiler) [📖](https://docs.rs/melody_compiler)
- `melody_cli` - A CLI wrapping the Melody compiler [📦](https://crates.io/crates/melody_cli) [📖](https://docs.rs/crate/melody_cli)
- `melody_wasm` - WASM binding for the Melody compiler

## Extensions

- [VSCode](https://marketplace.visualstudio.com/items?itemName=yoavlavi.melody)

## Performance

Last measured on V0.13.3

Measured on an 8 core 2021 MacBook Pro 14-inch, Apple M1 Pro using [criterion](https://github.com/bheisler/criterion.rs):

- 8 lines:

  ```
  compiler/normal (8 lines)
                          time:   [3.6560 us 3.6596 us 3.6644 us]
  slope  [3.6560 us 3.6644 us] R^2            [0.9999367 0.9999233]
  mean   [3.6577 us 3.6676 us] std. dev.      [3.2234 ns 11.399 ns]
  median [3.6549 us 3.6674 us] med. abs. dev. [642.70 ps 12.973 ns]
  ```

- 1M lines:

  ```
  compiler/long input (1M lines)
                          time:   [345.99 ms 348.85 ms 351.91 ms]
  mean   [345.99 ms 351.91 ms] std. dev.      [2.8317 ms 6.3397 ms]
  median [344.55 ms 352.85 ms] med. abs. dev. [893.36 us 8.5853 ms]
  ```

- Deeply nested:

  ```
  compiler/deeply nested
                          time:   [4.8259 us 4.8330 us 4.8399 us]
  slope  [4.8259 us 4.8399 us] R^2            [0.9998793 0.9998830]
  mean   [4.8259 us 4.8476 us] std. dev.      [7.6412 ns 24.306 ns]
  median [4.8234 us 4.8484 us] med. abs. dev. [4.1349 ns 30.340 ns]
  ```

To reproduce, run `cargo benchmark`

## Future Feature Status

🐣 - Partially implemented

❌ - Not implemented

❔ - Unclear what the syntax will be

❓ - Unclear whether this will be implemented

| Melody                              | Regex                 | Status      |
| ----------------------------------- | --------------------- | ----------- |
| `not "A";`                          | `[^A]`                | 🐣          |
| variables / macros                  |                       | 🐣          |
| file watcher                        |                       | ❌          |
| TS / JS build step                  |                       | ❌          |
| multiline groups in REPL            |                       | ❌          |
| `flags: global, multiline, ...`     | `/.../gm...`          | ❔          |
| (?)                                 | `\#`                  | ❔          |
| (?)                                 | `\k<name>`            | ❔          |
| (?)                                 | `\p{...}`             | ❔          |
| (?)                                 | `\P{...}`             | ❔          |
| (?)                                 | `\uYYYY`              | ❔          |
| (?)                                 | `\xYY`                | ❔          |
| (?)                                 | `\ddd`                | ❔          |
| (?)                                 | `\cY`                 | ❔          |
| (?)                                 | `$1`                  | ❔          |
| (?)                                 | <code>$\`</code>      | ❔          |
| (?)                                 | `$&`                  | ❔          |
| (?)                                 | `x20`                 | ❔          |
| (?)                                 | `x{06fa}`             | ❔          |
| `any of "a", "b", "c"` *            | `[abc]`               | ❓          |
| multiple ranges *                   | `[a-zA-Z0-9]`         | ❓          |
| regex optimization                  |                       | ❓          |
| standard library / patterns         |                       | ❓          |
| reverse compiler                    |                       | ❓          |

\* these are expressable in the current syntax using other methods
