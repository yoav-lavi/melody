---
sidebar_position: 6
---

# Syntax

## Keywords

- `of` - used after a number or a range and before a sequence to be matched, e.g. `5 of "A";`, equivalent to regex `{5}`
- `to` - used to create a range (either as a quantifier or as a character range), e.g. `5 to 9`, equivalent to regex `{5,9}` if before an `of` or `[5-9]` otherwise
- `capture` - used to open a `capture` or named `capture` block, equivalent to regex `(...)`
- `match` - used to open a `match` block, equivalent to regex `(?:...)`
- `either` - used to open an `either` block, equivalent to regex `(?:...|...)`
- `some` - used with `of` to express 1 or more of a pattern, equivalent to regex `+`
- `over` - used with `of` to express more than an amount of a pattern, equivalent to regex `{6,}` (assuming `over 5 of ...`)
- `option` - used with `of` to express 0 or 1 of a pattern, equivalent to regex `?`
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
- `<space>` - equivalent to regex ` `
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

- <code>`...`</code> - added directly to the output without any escaping

## Comments

- `/* ... */`, `// ...` - used to mark comments (note: `// ...` comments must be on separate line)
