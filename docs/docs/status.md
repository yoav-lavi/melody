---
sidebar_position: 7
---

# Feature Status

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
| `// comment`                        |                       | ✅          |
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
| <code>`\``;</code>                  | <code>\`</code>       | ✅          |
| support non alphanumeric characters |                       | ✅          |
| output to file                      |                       | ✅          |
| no color output                     |                       | ✅          |
| `<char>`                            | `.`                   | ✅          |
| `some of`                           | `+`                   | ✅          |
| syntax highlighting extension       |                       | ✅          |
| `over 5 of "A";`                    | `A{6,}`               | ✅          |
| `not <whitespace>;`                 | `\S`                  | ✅          |
| `not <digit>;`                      | `\D`                  | ✅          |
| `not <word>;`                       | `\W`                  | ✅          |
| WASM binding                        |                       | ✅          |
| Rust crate                          |                       | ✅          |
| `option of`                         | `?`                   | ✅          |
| `any of`                            | `*`                   | ✅          |
| `either { ...; ...; }`              | `(...\|...)`          | ✅          |
| tests                               |                       | ✅          |
| auto escape for literals            |                       | ✅          |
| `ahead { ... }`                     | `(?=...)`             | ✅          |
| `behind { ... }`                    | `(?<=...)`            | ✅          |
| `not ahead { ... }`                 | `(?!...)`             | ✅          |
| `not behind { ... }`                | `(?<!...)`            | ✅          |
| enforce group close                 |                       | ❌          |
| `<backspace>`                       | `[\b]`                | ❌          |
| file watcher                        |                       | ❌          |
| nested groups                       | `(...(...))`          | ❌          |
| multiple ranges                     | `[a-zA-Z0-9]`         | ❌          |
| general cleanup and modules         |                       | ❌          |
| TS / JS build step                  |                       | ❌          |
| more robust parsing mechanism (ast) |                       | ❌          |
| `not "A";`                          | `[^A]`                | ❔          |
| `flags: global, multiline, ...`     | `/.../gm...`          | ❔          |
| `/* comment */`                     |                       | ❔          |
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
