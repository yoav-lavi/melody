# RRX

RRX (Readable Regular Expressions) is a language designed to compile to and maintain a 1-1 relationship with regular expressions, while being more readable and maintainable. 

## Examples

```
16 of capture melody {
  na;
}

2 of capture {
  <space>;
  batman;
}
```

Turns into

```
/(?<melody>na){16}(\sbatman){2}/
```

## Feature Status

| RRX                                 | Regex                 | Implemented | Unclear      |
| ----------------------------------- | --------------------- | ----------- | ------------ |
| `5 of A;`                           | `A{5}`                | [x]         | [ ]          |
| `5 to 7 of A;`                      | `A{5,7}`              | [x]         | [ ]          |
| `capture { ... }`                   | `(...)`               | [x]         | [ ]          |
| `capture name { ... }`              | `(?<name>...)`        | [x]         | [ ]          |
| `match { ... }`                     | `(?:...)`             | [x]         | [ ]          |
| `<space>;`                          | `\s`                  | [x]         | [ ]          |
| `A to Z;`                           | `[A-Z]`               | [x]         | [ ]          |
| `a to z;`                           | `[a-z]`               | [x]         | [ ]          |
| `0 to 9;`                           | `[0-9]`               | [x]         | [ ]          |
| `// comment`                        |                       | [ ]         | [ ]          |
| `not A;`                            | `[^A]`                | [ ]         | [ ]          |
| `not before ...`                    | `(?!...)`             | [ ]         | [ ]          |
| `not after ...`                     | `(?<!...)`            | [ ]         | [ ]          |
| `before ...`                        | `(?=...)`             | [ ]         | [ ]          |
| `after ...`                         | `(?<=...)`            | [ ]         | [ ]          |
| `start;`                            | `^`                   | [ ]         | [ ]          |
| `end;`                              | `$`                   | [ ]         | [ ]          |
| `not <space>;`                      | `\S`                  | [ ]         | [ ]          |
| `<newline>;`                        | `\n`                  | [ ]         | [ ]          |
| `<tab>;`                            | `\t`                  | [ ]         | [ ]          |
| `<return>;`                         | `\r`                  | [ ]         | [ ]          |
| `<feed>;`                           | `\f`                  | [ ]         | [ ]          |
| `<null>;`                           | `\0`                  | [ ]         | [ ]          |
| `<digit>;`                          | `\d`                  | [ ]         | [ ]          |
| `not <digit>;`                      | `\D`                  | [ ]         | [ ]          |
| `<word>;`                           | `\w`                  | [ ]         | [ ]          |
| `not <word>;`                       | `\W`                  | [ ]         | [ ]          |
| `<vertical>;`                       | `\v`                  | [ ]         | [ ]          |
| `<backspace>`                       | `[\b]`                | [ ]         | [ ]          |
| `some of`                           | `+`                   | [ ]         | [ ]          |
| `"...";` (raw)                      | ...                   | [ ]         | [ ]          |
| `<quote>;`                          | `\"`                  | [ ]         | [ ]          |
| `flags: global, multiline, ...`     | `/.../gm...`          | [ ]         | [ ]          |
| nested groups                       | `(...(...))`          | [ ]         | [ ]          |
| multiple ranges                     | `[a-zA-Z0-9]`         | [ ]         | [ ]          |
| tests                               |                       | [ ]         | [ ]          |
| general cleanup and modules         |                       | [ ]         | [ ]          |
| auto escape for non RRX patterns    |                       | [ ]         | [ ]          |
| support non alphanumeric characters |                       | [ ]         | [ ]          |
| syntax highlighting extension       |                       | [ ]         | [ ]          |
| `/* comment */`                     |                       | [ ]         | [x]          |
| `over 4 of A;`                      | `A{5,}`               | [ ]         | [x]          |
| `maybe of`                          | `?`                   | [ ]         | [x]          |
| `maybe some of`                     | `*`                   | [ ]         | [x]          |
| `either of ..., ...`                | `\|`                  | [ ]         | [x]          |
| `any of a, b, c`                    | `[abc]`               | [ ]         | [x]          |
| variables / macros                  |                       | [ ]         | [x]          |
| regex optimization                  |                       | [ ]         | [x]          |
| standard library / patterns         |                       | [ ]         | [x]          |
| `character;` (`char`?)              | `.`                   | [ ]         | [x]          |
| (?)                                 | `*?`                  | [ ]         | [x]          |
| (?)                                 | `\#`                  | [ ]         | [x]          |
| (?)                                 | `\k<name>`            | [ ]         | [x]          |
| (?)                                 | `\p{...}`             | [ ]         | [x]          |
| (?)                                 | `\P{...}`             | [ ]         | [x]          |
| (?)                                 | `\uYYYY`              | [ ]         | [x]          |
| (?)                                 | `\xYY`                | [ ]         | [x]          |
| (?)                                 | `\ddd`                | [ ]         | [x]          |
| (?)                                 | `\cY`                 | [ ]         | [x]          |
| (?)                                 | `\b`                  | [ ]         | [x]          |
| (?)                                 | `\B`                  | [ ]         | [x]          |
| (?)                                 | `$1`                  | [ ]         | [x]          |
| (?)                                 | <code>$`</code>       | [ ]         | [x]          |
| (?)                                 | `$&`                  | [ ]         | [x]          |
| (?)                                 | `x20`                 | [ ]         | [x]          |
| (?)                                 | `x{06fa}`             | [ ]         | [x]          |
               

  