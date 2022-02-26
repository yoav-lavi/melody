#![macro_use]

macro_rules! parse_error {
    ($lexer: expr, line:ident) => {
        return Err(create_parse_error($lexer.slice(), $lexer.source(), $line));
    };
}
