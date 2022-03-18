<p align="center">
    <img alt="Melody Logo" height="250px" src="https://user-images.githubusercontent.com/14347895/159065887-51a2d948-ae6f-48c4-9dd2-1ee69e76b19f.png">
</p>

<p align="center">
The Melody language compiler
</p>

## Install

```toml
[dependencies]
melody_compiler = "0.13.7"
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
- [Language Documentation](https://yoav-lavi.github.io/melody/book/)