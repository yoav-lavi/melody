mod errors;
mod lexer;
mod utils;

pub use errors::ParseError;
use lexer::{lex, tokens::Token};
use logos::Lexer;
use utils::format_regex;

/**
Compiles Melody source code to a regular expression

see also: [ParseError]

# Example

```rust
use melody_compiler::compiler;

let source = r#"1 to 5 of "A";"#;
let output = compiler(&source);

assert_eq!(output.unwrap(), "/A{1,5}/");
```
*/
pub fn compiler(source: &str) -> Result<String, ParseError> {
    let mut lexer = lex(source);

    let mut in_group = false;

    let mut in_either = false;

    let mut in_modifier = 0;

    let mut line: u16 = 0;

    let mut quantifier = None;

    let mut group_quantifier = None;

    let mut store: Vec<String> = Vec::new();

    let mut output = String::new();

    while let Some(token) = lexer.next() {
        let in_block = in_group || in_either;

        let formatted_token = match token {
            // raw
            Token::LiteralDouble(pattern) | Token::LiteralSingle(pattern) | Token::Raw(pattern) => {
                let group = pattern.chars().count() != 1;
                handle_quantifier(pattern, quantifier.clone(), group)
            }

            // ranges
            Token::LowercaseRange(range)
            | Token::UppercaseRange(range)
            | Token::NumericRange(range) => handle_quantifier(range, quantifier.clone(), false),

            // groups
            Token::Capture => {
                if in_block {
                    return Err(create_parse_error(lexer, line));
                }
                group_quantifier = quantifier;
                quantifier = None;
                in_group = true;
                Some(String::from("("))
            }
            Token::NamedCapture(name) => {
                if in_block {
                    return Err(create_parse_error(lexer, line));
                }
                group_quantifier = quantifier;
                quantifier = None;
                in_group = true;
                Some(format!("(?<{name}>"))
            }
            Token::Match => {
                if in_block {
                    return Err(create_parse_error(lexer, line));
                }
                group_quantifier = quantifier;
                quantifier = None;
                in_group = true;
                Some(String::from("(?:"))
            }
            Token::Ahead => {
                if in_block {
                    return Err(create_parse_error(lexer, line));
                }
                group_quantifier = quantifier;
                quantifier = None;
                in_group = true;
                Some(String::from("(?="))
            }
            Token::NotAhead => {
                if in_block {
                    return Err(create_parse_error(lexer, line));
                }
                group_quantifier = quantifier;
                quantifier = None;
                in_group = true;
                Some(String::from("(?!"))
            }
            Token::Behind => {
                if in_block {
                    return Err(create_parse_error(lexer, line));
                }
                group_quantifier = quantifier;
                quantifier = None;
                in_group = true;
                Some(String::from("(?<="))
            }
            Token::NotBehind => {
                if in_block {
                    return Err(create_parse_error(lexer, line));
                }
                group_quantifier = quantifier;
                quantifier = None;
                in_group = true;
                Some(String::from("(?<!"))
            }
            Token::Either => {
                if in_block {
                    return Err(create_parse_error(lexer, line));
                }
                group_quantifier = quantifier;
                quantifier = None;
                in_either = true;
                None
            }
            Token::BlockEnd => {
                if in_group {
                    in_group = false;
                    let current_group_quantifier = group_quantifier;
                    group_quantifier = None;
                    handle_quantifier(String::from(")"), current_group_quantifier, false)
                } else if in_either {
                    in_either = false;
                    let inner_expressions = store.join("|");
                    store.clear();
                    let current_group_quantifier = group_quantifier;
                    group_quantifier = None;
                    handle_quantifier(
                        format!("(?:{})", inner_expressions),
                        current_group_quantifier,
                        false,
                    )
                } else {
                    None
                }
            }

            // modifiers
            Token::QuantifierExpression(quantity) => {
                if in_modifier > 0 {
                    return Err(create_parse_error(lexer, line));
                }
                in_modifier = 2;
                quantifier = Some(quantity);
                None
            }
            Token::OpenRangeExpression(range) | Token::RangeExpression(range) => {
                if in_modifier > 0 {
                    return Err(create_parse_error(lexer, line));
                }
                in_modifier = 2;
                quantifier = Some(range);
                None
            }
            Token::Semicolon => {
                quantifier = None;
                None
            }
            Token::SomeExpression => {
                if in_modifier > 0 {
                    return Err(create_parse_error(lexer, line));
                }
                in_modifier = 2;
                quantifier = Some(String::from("+"));
                None
            }
            Token::OptionExpression => {
                if in_modifier > 0 {
                    return Err(create_parse_error(lexer, line));
                }
                in_modifier = 2;
                quantifier = Some(String::from("?"));
                None
            }
            Token::AnyExpression => {
                if in_modifier > 0 {
                    return Err(create_parse_error(lexer, line));
                }
                in_modifier = 2;
                quantifier = Some(String::from("*"));
                None
            }

            // direct replacements
            Token::StartSymbol => handle_quantifier(String::from("^"), quantifier.clone(), false),
            Token::EndSymbol => handle_quantifier(String::from("$"), quantifier.clone(), false),
            Token::WhitespaceSymbol => {
                handle_quantifier(String::from("\\s"), quantifier.clone(), false)
            }
            Token::NotWhitespaceSymbol => {
                handle_quantifier(String::from("\\S"), quantifier.clone(), false)
            }

            Token::SpaceSymbol => handle_quantifier(String::from(" "), quantifier.clone(), false),
            Token::NewlineSymbol => {
                handle_quantifier(String::from("\\n"), quantifier.clone(), false)
            }
            Token::TabSymbol => handle_quantifier(String::from("\\t"), quantifier.clone(), false),
            Token::ReturnSymbol => {
                handle_quantifier(String::from("\\r"), quantifier.clone(), false)
            }
            Token::FeedSymbol => handle_quantifier(String::from("\\f"), quantifier.clone(), false),
            Token::NullSymbol => handle_quantifier(String::from("\\0"), quantifier.clone(), false),
            Token::DigitSymbol => handle_quantifier(String::from("\\d"), quantifier.clone(), false),
            Token::NotDigitSymbol => {
                handle_quantifier(String::from("\\D"), quantifier.clone(), false)
            }
            Token::WordSymbol => handle_quantifier(String::from("\\w"), quantifier.clone(), false),
            Token::NotWordSymbol => {
                handle_quantifier(String::from("\\W"), quantifier.clone(), false)
            }
            Token::AlphabetSymbol => {
                handle_quantifier(String::from("[a-zA-Z]"), quantifier.clone(), false)
            }
            Token::VerticalSymbol => {
                handle_quantifier(String::from("\\v"), quantifier.clone(), false)
            }
            Token::CharSymbol => handle_quantifier(String::from("."), quantifier.clone(), false),

            // warning and error related
            Token::NewLine => {
                line += 1;
                None
            }
            _ => return Err(create_parse_error(lexer, line)),
        };

        if let Some(formatted_token) = formatted_token {
            if in_either {
                store.push(formatted_token);
            } else {
                output.push_str(&formatted_token);
            }
        }

        if in_modifier > 0 {
            in_modifier -= 1
        }
    }

    Ok(format_regex(&output, None))
}

fn create_parse_error(lexer: Lexer<Token>, line: u16) -> ParseError {
    ParseError::new(lexer.slice(), lexer.source(), usize::from(line))
}

fn handle_quantifier(source: String, quantifier: Option<String>, group: bool) -> Option<String> {
    if let Some(quantifier) = quantifier {
        let formatted_source = if group {
            format!("(?:{source}){quantifier}")
        } else {
            [source, quantifier].join("")
        };
        Some(formatted_source)
    } else {
        Some(source)
    }
}
