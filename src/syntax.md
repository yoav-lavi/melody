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
- `<space>` - matches a space character. equivalent to regex ` `
- `<whitespace>` - matches any kind of whitespace character. equivalent to regex `\s` or `[ \t\n\v\f\r]`
- `<newline>` - matches a newline character. equivalent to regex `\n`
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

## Unicode Categories

Note: these are not supported when testing in the CLI (`-t` or `-f`) as the regex engine used does not support unicode categories. These require using the `u` flag.

- `<category::letter>` - any kind of letter from any language
  - `<category::lowercase_letter>` - a lowercase letter that has an uppercase variant
  - `<category::uppercase_letter>` - an uppercase letter that has a lowercase variant.
  - `<category::titlecase_letter>` - a letter that appears at the start of a word when only the first letter of the word is capitalized
  - `<category::cased_letter>` - a letter that exists in lowercase and uppercase variants
  - `<category::modifier_letter>` - a special character that is used like a letter
  - `<category::other_letter>` - a letter or ideograph that does not have lowercase and uppercase variants
- `<category::mark>` - a character intended to be combined with another character (e.g. accents, umlauts, enclosing boxes, etc.)
  - `<category::non_spacing_mark>` - a character intended to be combined with another character without taking up extra space (e.g. accents, umlauts, etc.)
  - `<category::spacing_combining_mark>` - a character intended to be combined with another character that takes up extra space (vowel signs in many Eastern languages)
  - `<category::enclosing_mark>` - a character that encloses the character it is combined with (circle, square, keycap, etc.)
- `<category::separator>` - any kind of whitespace or invisible separator
  - `<category::space_separator>` - a whitespace character that is invisible, but does take up space
  - `<category::line_separator>` - line separator character U+2028
  - `<category::paragraph_separator>` - paragraph separator character U+2029
- `<category::symbol>` - math symbols, currency signs, dingbats, box-drawing characters, etc
  - `<category::math_symbol>` - any mathematical symbol
  - `<category::currency_symbol>` - any currency sign
  - `<category::modifier_symbol>` - a combining character (mark) as a full character on its own
  - `<category::other_symbol>` - various symbols that are not math symbols, currency signs, or combining characters
- `<category::number>` - any kind of numeric character in any script
  - `<category::decimal_digit_number>` - a digit zero through nine in any script except ideographic scripts
  - `<category::letter_number>` - a number that looks like a letter, such as a Roman numeral
  - `<category::other_number>` - a superscript or subscript digit, or a number that is not a digit 0–9 (excluding numbers from ideographic scripts)
- `<category::punctuation>` - any kind of punctuation character
  - `<category::dash_punctuation>` - any kind of hyphen or dash
  - `<category::open_punctuation>` - any kind of opening bracket
  - `<category::close_punctuation>` -  any kind of closing bracket
  - `<category::initial_punctuation>` - any kind of opening quote
  - `<category::final_punctuation>` - any kind of closing quote
  - `<category::connector_punctuation>` - a punctuation character such as an underscore that connects words
  - `<category::other_punctuation>` - any kind of punctuation character that is not a dash, bracket, quote or connectors
- `<category::other>` - invisible control characters and unused code points
  - `<category::control>` - an ASCII or Latin-1 control character: 0x00–0x1F and 0x7F–0x9F
  - `<category::format>` - invisible formatting indicator
  - `<category::private_use>` - any code point reserved for private use
  - `<category::surrogate>` - one half of a surrogate pair in UTF-16 encoding
  - `<category::unassigned>` - any code point to which no character has been assigned

These descriptions are from [regular-expressions.info](https://www.regular-expressions.info/unicode.html)

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

## Extras

- `/* ... */`, `// ...` - used to mark comments (note: `// ...` comments must be on separate line)
