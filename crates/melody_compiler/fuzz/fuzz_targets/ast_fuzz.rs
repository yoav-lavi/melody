#![no_main]
use libfuzzer_sys::fuzz_target;
use melody_compiler::ast::enums::*;
use melody_compiler::to_regex;

fuzz_target!(|data: Vec<Node>| {
    drop(to_regex(&data));
});
