---
sidebar_position: 6
---

# Syntax


## Quantifiers

- `... of` - used to express a specific amount of a pattern. equivalent to regex `{5}` (assuming `5 of ...`)
- `... to ... of` - used to express an amount within a range of a pattern. equivalent to regex `{5,9}` (assuming `5 to 9 of ...`)
- `over ... of` - used to express more than an amount of a pattern. equivalent to regex `{6,}` (assuming `over 5 of ...`)
- `some of` - used to express 1 or more of a pattern. equivalent to regex `+`
- `any of` - used to express 0 or more of a pattern. equivalent to regex `*`
- `option of` - used to express 0 or 1 of a pattern. equivalent to regex `?`

All quantifiers can be preceded by `lazy` to match the least amount of characters rather than the most characters (greedy). Equivalent to regex  `+?`, `*?`, etc.

## Symbols

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

## Special Symbols

- `<start>` - matches the start of the string. equivalent to regex `^`
- `<end>` - matches the end of the string. equivalent to regex `$`

## Character Ranges

- `... to ...` - used with digits or alphabetic characters to express a character range. equivalent to regex `[5-9]` (assuming `5 to 9`) or `[a-z]` (assuming `a to z`)

## Literals

- `"..."` or `'...'` - used to mark a literal part of the match. Melody will automatically escape characters as needed. Quotes (of the same kind surrounding the literal) should be escaped

## Raw

- <code>\`...\`</code> - added directly to the output without any escaping

## Groups

- `capture` - used to open a `capture` or named `capture` block. captured patterns are later available in the list of matches (either positional or named). equivalent to regex `(...)`
- `match` - used to open a `match` block, matches the contents without capturing. equivalent to regex `(?:...)`
- `either` - used to open an `either` block, matches one of the statements within the block. equivalent to regex `(?:...|...)`

## Assertions

- `ahead` - used to open an `ahead` block. equivalent to regex `(?=...)`. use after an expression
- `behind` - used to open an `behind` block. equivalent to regex `(?<=...)`. use before an expression

Assertions can be preceeded by `not` to create a negative assertion (equivalent to regex `(?!...)`, `(?<!...)`)

## Variables

- `let .variable_name = { ... }` - defines a variable from a block of statements. can later be used with `.variable_name`. Variables must be declared before being used. Variable declarations cannot reference the created variable (recursion). Variable invocations cannot be quantified directly, use a group if you want to quantify a variable invocation
  
  example:

  ```
  let .a_and_b = {
    "a";
    "b";
  }
  
  .a_and_b;
  "c";

  // abc

## Extras

- `/* ... */`, `// ...` - used to mark comments (note: `// ...` comments must be on separate line)