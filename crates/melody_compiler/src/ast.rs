pub mod enums;
pub mod source_to_ast;

mod consts;
mod error_messages;
mod ident_parser;
mod utils;

pub use self::source_to_ast::to_ast;
use std::collections::HashSet;

lazy_static! {
    static ref RESERVED_CHARS: HashSet<char> = HashSet::from([
        '[', ']', '(', ')', '{', '}', '*', '+', '?', '|', '^', '$', '.', '-', '\\',
    ]);
}
