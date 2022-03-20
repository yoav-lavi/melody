#![no_main]
use libfuzzer_sys::fuzz_target;
use melody_compiler::{ast::enums::*, ast_to_regex};

fuzz_target!(|data: MelodyAst| {
    drop(ast_to_regex(&data));
});
