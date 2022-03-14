<p align="center">
    <img alt="Melody Logo" height="250px" src="https://user-images.githubusercontent.com/14347895/157926614-8434c590-e810-494c-ac9d-3657e9aa4583.png">
</p>

<p align="center">
The Melody language compiler
</p>

## Install

```toml
[dependencies]
melody_compiler = "0.13.6"
```

## Usage

```rust
use melody_compiler::compiler;

let source = r#"1 to 5 of "A";"#;
let output = compiler(source);

assert_eq!(output.unwrap(), "A{1,5}");
```

## Links

- [docs.rs](https://docs.rs/melody_compiler/)
- [Language Documentation](https://yoav-lavi.github.io/melody/)