#![no_main]
use libfuzzer_sys::fuzz_target;
use melody_compiler::{ast::types::ast::*, ast_to_regex};

fuzz_target!(|data: MelodyAst| {
    drop(ast_to_regex(&data));
});
