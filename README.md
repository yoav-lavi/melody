<p align="center">
  <img width="520" alt="code example" src="https://user-images.githubusercontent.com/14347895/154124756-ddbd3c84-f8b2-45bd-b624-2c510482c4e2.png">
</p>

Melody is a language designed to compile to and maintain a 1-1 relationship with regular expressions, while being more readable and maintainable.

The current goal is supporting the JavaScript implementation of regular expressions.

⚠️ Melody is very new and is unstable at the moment ⚠️

## Examples

Note: these are for the currently supported syntax and may change

### Batman Theme

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
/(?:na){16}(?: batman){2}/
```

### Twitter Hashtag

```rust
"#";
some of <word>;

// #melody
```

Turns into

```regex
/#\w+/
```

### Introductory Courses

```rust
some of <word>;
<space>;
"1";
2 of <digit>;

// classname 1xx
```

Turns into

```regex
/\w+ 1\d{2}/
```

### Indented Code (2 spaces)

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
/(?: {2})+.+;/
```

### Semantic Versions

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
/^v?(?<major>\d+)\.(?<minor>\d+)\.(?<patch>\d+)$/
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

## Keywords

- `of` - used after a number or a range and before a sequence to be matched, e.g. `5 of "A";`, equivalent to regex `{5}`
- `to` - used to create a range (either as a quantifier or as a character range), e.g. `5 to 9`, equivalent to regex `{5,9}` if before an `of` or `[5-9]` otherwise
- `capture` - used to open a `capture` or named `capture` block, equivalent to regex `(...)`
- `match` - used to open a `match` block, equivalent to regex `(?:...)`
- `some` - used with `of` to express 1 or more of a pattern, equivalent to regex `+`
- `over` - used with `of` to express more than an amount of a pattern, equivalent to regex `{6,}` (assuming `over 5 of ...`)
- `option` - used with `of` to express 0 or 1 of a pattern, equivalent to regex `?`
- `either` - used to open an `either` block, equivalent to regex `(?:...|...)`
- `any` - used with `of` to express 0 or more of a pattern, equivalent to regex `*`
- `ahead` - used to open an `ahead` block, equivalent to regex `(?=...)`. use after an expression
- `not ahead` - used to open a `not ahead` block, equivalent to regex `(?!...)`. use after an expression
- `behind` - used to open an `behind` block, equivalent to regex `(?<=...)`. use before an expression
- `not behind` - used to open a `not behind` block, equivalent to regex `(?<!...)`. use before an expression

## Symbols

- `<start>` - matches the start of the string, equivalent to regex `^`
- `<end>` - matches the end of the string, equivalent to regex `$`
- `<char>` - matches a single character, equivalent to regex `.`
- `<whitespace>` - equivalent to regex `\s`
- `not <whitespace>` - equivalent to regex `\S`
- `<newline>` - equivalent to regex `\n`
- `<tab>` - equivalent to regex `\t`
- `<return>` - equivalent to regex `\r`
- `<feed>` - equivalent to regex `\f`
- `<null>` - equivalent to regex `\0`
- `<digit>` - equivalent to regex `\d`
- `not <digit>` - equivalent to regex `\D`
- `<vertical>` - equivalent to regex `\v`
- `<word>` - equivalent to regex `\w`
- `not <word>` - equivalent to regex `\W`
- `<alphabet>` - equivalent to regex `[a-zA-Z]`
- `<boundary>` - equivalent to regex `\b`
- `<backspace>` - equivalent to regex `[\b]`

## Literals

- `"..."` or `'...'` - used to mark a literal part of the match. Melody will automatically escape characters as needed. Quotes (of the same kind surrounding the literal) should be escaped

## Raw

- <code>\`...\`</code> - added directly to the output without any escaping

## Extras

- `/* ... */`, `// ...` - used to mark comments (note: `// ...` comments must be on separate line)

## File Extension

The Melody file extension is `.mdy`

## Crates

- `melody_compiler` - The Melody compiler [📦](https://crates.io/crates/melody_compiler) [📖](https://docs.rs/melody_compiler/)
- `melody_cli` - A CLI wrapping the Melody compiler [📦](https://crates.io/crates/melody_cli) [📖](https://docs.rs/crate/melody_cli)
- `melody_wasm` - WASM binding for the Melody compiler

## Extensions

- [VSCode](https://marketplace.visualstudio.com/items?itemName=yoavlavi.melody)


## Performance

Measured on a 8 core 2021 MacBook Pro 14-inch, Apple M1 Pro using [hyperfine](https://github.com/sharkdp/hyperfine), compiling a 2.3M LOC file via the Melody CLI (the file was made using a repetition of the examples in this repository):

```
Time (mean ± σ):      1.002 s ±  0.010 s    [User: 0.905 s, System: 0.093 s]
Range (min … max):    0.984 s …  1.016 s    20 runs
```

For real world usage (on similar hardware), expect less than 1 ms (0.8 ms on a 100 LOC file, but hyperfine might be inaccurate under 5 ms)

## Feature Status

✅ - Implemented

🐣 - Partially implemented

❌ - Not implemented

❓ - Unclear whether this will be implemented

❔ - Unclear what the syntax will be

| Melody                              | Regex                 | Status      |
| ----------------------------------- | --------------------- | ----------- |
| `5 of "hello";`                     | `(?:hello){5}`        | ✅          |
| `5 to 7 of "A";`                    | `A{5,7}`              | ✅          |
| `capture { ... }`                   | `(...)`               | ✅          |
| `capture name { ... }`              | `(?<name>...)`        | ✅          |
| `match { ... }`                     | `(?:...)`             | ✅          |
| `<whitespace>;`                     | `\s`                  | ✅          |
| `<space>;`                          | ` `                   | ✅          |
| `A to Z;`                           | `[A-Z]`               | ✅          |
| `a to z;`                           | `[a-z]`               | ✅          |
| `0 to 9;`                           | `[0-9]`               | ✅          |
| `<start>;`                          | `^`                   | ✅          |
| `<end>;`                            | `$`                   | ✅          |
| `<newline>;`                        | `\n`                  | ✅          |
| `<tab>;`                            | `\t`                  | ✅          |
| `<return>;`                         | `\r`                  | ✅          |
| `<feed>;`                           | `\f`                  | ✅          |
| `<null>;`                           | `\0`                  | ✅          |
| `<digit>;`                          | `\d`                  | ✅          |
| `<vertical>;`                       | `\v`                  | ✅          |
| `<word>;`                           | `\w`                  | ✅          |
| `<alphabet>;`                       | `[a-zA-Z]`            | ✅          |
| `"...";` (literal)                  | `...`                 | ✅          |
| `'...';` (literal)                  | `...`                 | ✅          |
| <code>\`...\`;</code> (raw)         | `...`                 | ✅          |
| `'\'';`                             | `'`                   | ✅          |
| `"\"";`                             | `"`                   | ✅          |
| <code>`\\\``;</code>                | <code>\`</code>       | ✅          |
| support non alphanumeric characters |                       | ✅          |
| output to file                      |                       | ✅          |
| no color output                     |                       | ✅          |
| `<char>`                            | `.`                   | ✅          |
| `some of`                           | `+`                   | ✅          |
| syntax highlighting extension       |                       | ✅          |
| `over 5 of "A";`                    | `A{6,}`               | ✅          |
| `not <digit>;`                      | `\D`                  | ✅          |
| `not <whitespace>;`                 | `\S`                  | ✅          |
| `not <word>;`                       | `\W`                  | ✅          |
| WASM binding                        |                       | ✅          |
| Rust crate                          |                       | ✅          |
| `option of`                         | `?`                   | ✅          |
| `any of`                            | `*`                   | ✅          |
| `either { ...; ...; }`              | `(?:...\|...)`        | ✅          |
| tests                               |                       | ✅          |
| auto escape for literals            |                       | ✅          |
| `ahead { ... }`                     | `(?=...)`             | ✅          |
| `behind { ... }`                    | `(?<=...)`            | ✅          |
| `not ahead { ... }`                 | `(?!...)`             | ✅          |
| `not behind { ... }`                | `(?<!...)`            | ✅          |
| `/* comment */`                     |                       | ✅          |
| enforce group close                 |                       | ✅          |
| nested groups                       | `(...(...))`          | ✅          |
| general cleanup and modules         |                       | ✅          |
| more robust parsing mechanism (ast) |                       | ✅          |
| `<backspace>`                       | `[\b]`                | ✅          |
| `<boundary>`                        | `\b`                  | ✅          |
| `// comment`                        |                       | ✅          |
| `not "A";`                          | `[^A]`                | 🐣          |
| file watcher                        |                       | ❌          |
| multiple ranges                     | `[a-zA-Z0-9]`         | ❌          |
| TS / JS build step                  |                       | ❌          |
| multiline groups in REPL            |                       | ❌          |
| `flags: global, multiline, ...`     | `/.../gm...`          | ❔          |
| `any of "a", "b", "c"`              | `[abc]`               | ❔          |
| (?)                                 | `*?`                  | ❔          |
| (?)                                 | `\#`                  | ❔          |
| (?)                                 | `\k<name>`            | ❔          |
| (?)                                 | `\p{...}`             | ❔          |
| (?)                                 | `\P{...}`             | ❔          |
| (?)                                 | `\uYYYY`              | ❔          |
| (?)                                 | `\xYY`                | ❔          |
| (?)                                 | `\ddd`                | ❔          |
| (?)                                 | `\cY`                 | ❔          |
| (?)                                 | `\b`                  | ❔          |
| (?)                                 | `\B`                  | ❔          |
| (?)                                 | `$1`                  | ❔          |
| (?)                                 | <code>$\`</code>      | ❔          |
| (?)                                 | `$&`                  | ❔          |
| (?)                                 | `x20`                 | ❔          |
| (?)                                 | `x{06fa}`             | ❔          |
| variables / macros                  |                       | ❓          |
| regex optimization                  |                       | ❓          |
| standard library / patterns         |                       | ❓          |
| reverse compiler                    |                       | ❓          |
