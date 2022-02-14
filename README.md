# RRX

<p align="center">
  <img alt="code example" src="https://user-images.githubusercontent.com/14347895/153910592-8a25c713-82f8-4fb4-ba89-073f517f0a3d.png" width="445px" style="">
</p>

RRX (Readable Regular Expressions) is a language designed to compile to and maintain a 1-1 relationship with regular expressions, while being more readable and maintainable.

The current goal is supporting the JavaScript implementation of regular expressions.

## Examples

```coffeescript
16 of capture melody {
  na;
}

2 of capture {
  <space>;
  batman;
}
```

Turns into

```regex
/(?<melody>na){16}(\sbatman){2}/
```

## Keywords (implemented only)

- `of` - used after a number or a range and before a sequence to be matched, e.g. `5 of A;`, equivalent to regex `{5}`
- `to` - used to create a range (either as a quantifier or as a character range), e.g. `5 to 9`, equivalent to regex `{5,9}` if before an `of` or `[5-9]` otherwise
- `capture` - used to open a `capture` or named `capture` block, equivalent to regex `(...)`
- `match` - used to open a `match` block, equivalent to regex `(?:...)`
- `start` - matches the start of the string, equivalent to regex `^`
- `end` - matches the start of the string, equivalent to regex `$`

## Symbols (implemented only)

- `<space>` - equavalent to regex `\s`
- `<newline>` - equavalent to regex `\n`
- `<tab>` - equavalent to regex `\t`
- `<return>` - equavalent to regex `\r`
- `<feed>` - equavalent to regex `\f`
- `<null>` - equavalent to regex `\n`
- `<digit>` - equavalent to regex `\d`
- `<vertical>` - equavalent to regex `\v`
- `<word>` - equavalent to regex `\w`

## Extras (implemented only)

- `//` - used to mark comments

## Feature Status

| RRX                                 | Regex                 | Implemented | Unclear      |
| ----------------------------------- | --------------------- | ----------- | ------------ |
| `5 of A;`                           | `A{5}`                | ✅          |              |
| `5 to 7 of A;`                      | `A{5,7}`              | ✅          |              |
| `capture { ... }`                   | `(...)`               | ✅          |              |
| `capture name { ... }`              | `(?<name>...)`        | ✅          |              |
| `match { ... }`                     | `(?:...)`             | ✅          |              |
| `<space>;`                          | `\s`                  | ✅          |              |
| `A to Z;`                           | `[A-Z]`               | ✅          |              |
| `a to z;`                           | `[a-z]`               | ✅          |              |
| `0 to 9;`                           | `[0-9]`               | ✅          |              |
| `// comment`                        |                       | ✅          |              |
| `start;`                            | `^`                   | ✅          |              |
| `end;`                              | `$`                   | ✅          |              |
| `<newline>;`                        | `\n`                  | ✅          |              |
| `<tab>;`                            | `\t`                  | ✅          |              |
| `<return>;`                         | `\r`                  | ✅          |              |
| `<feed>;`                           | `\f`                  | ✅          |              |
| `<null>;`                           | `\0`                  | ✅          |              |
| `<digit>;`                          | `\d`                  | ✅          |              |
| `<vertical>;`                       | `\v`                  | ✅          |              |
| `<word>;`                           | `\w`                  | ✅          |              |
| `not A;`                            | `[^A]`                |             |              |
| `not before ...`                    | `(?!...)`             |             |              |
| `not after ...`                     | `(?<!...)`            |             |              |
| `before ...`                        | `(?=...)`             |             |              |
| `after ...`                         | `(?<=...)`            |             |              |
| `not <space>;`                      | `\S`                  |             |              |
| `not <digit>;`                      | `\D`                  |             |              |
| `not <word>;`                       | `\W`                  |             |              |
| `<backspace>`                       | `[\b]`                |             |              |
| `some of`                           | `+`                   |             |              |
| `"...";` (raw)                      | ...                   |             |              |
| `<quote>;`                          | `\"`                  |             |              |
| nested groups                       | `(...(...))`          |             |              |
| multiple ranges                     | `[a-zA-Z0-9]`         |             |              |
| tests                               |                       |             |              |
| general cleanup and modules         |                       |             |              |
| auto escape for non RRX patterns    |                       |             |              |
| support non alphanumeric characters |                       |             |              |
| syntax highlighting extension       |                       |             |              |
| `flags: global, multiline, ...`     | `/.../gm...`          |             | ❓           |
| `/* comment */`                     |                       |             | ❓           |
| `over 4 of A;`                      | `A{5,}`               |             | ❓           |
| `maybe of`                          | `?`                   |             | ❓           |
| `maybe some of`                     | `*`                   |             | ❓           |
| `either of ..., ...`                | `\|`                  |             | ❓           |
| `any of a, b, c`                    | `[abc]`               |             | ❓           |
| variables / macros                  |                       |             | ❓           |
| regex optimization                  |                       |             | ❓           |
| standard library / patterns         |                       |             | ❓           |
| `character;` (`char`?)              | `.`                   |             | ❓           |
| (?)                                 | `*?`                  |             | ❓           |
| (?)                                 | `\#`                  |             | ❓           |
| (?)                                 | `\k<name>`            |             | ❓           |
| (?)                                 | `\p{...}`             |             | ❓           |
| (?)                                 | `\P{...}`             |             | ❓           |
| (?)                                 | `\uYYYY`              |             | ❓           |
| (?)                                 | `\xYY`                |             | ❓           |
| (?)                                 | `\ddd`                |             | ❓           |
| (?)                                 | `\cY`                 |             | ❓           |
| (?)                                 | `\b`                  |             | ❓           |
| (?)                                 | `\B`                  |             | ❓           |
| (?)                                 | `$1`                  |             | ❓           |
| (?)                                 | <code>$`</code>       |             | ❓           |
| (?)                                 | `$&`                  |             | ❓           |
| (?)                                 | `x20`                 |             | ❓           |
| (?)                                 | `x{06fa}`             |             | ❓           |

## Acknowledgments

RRX uses:

- [Logos](https://github.com/maciejhirsz/logos) [(license)](https://github.com/maciejhirsz/logos/blob/master/LICENSE-MIT)
- [Itertools](https://github.com/rust-itertools/itertools) [(license)](https://github.com/rust-itertools/itertools/blob/master/LICENSE-MIT)
- [Clap](https://github.com/clap-rs/clap) [(license)](https://github.com/clap-rs/clap/blob/master/LICENSE-MIT)
- [Colored](https://github.com/mackwic/colored) [(license)](https://github.com/mackwic/colored/blob/master/LICENSE)
