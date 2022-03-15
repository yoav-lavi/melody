#![no_main]
use libfuzzer_sys::fuzz_target;
use melody_compiler::{ast::enums::*, to_regex};

fuzz_target!(|data: Vec<Node>| {
    drop(to_regex(&data));
});
