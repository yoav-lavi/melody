// in a separate file due to rust-analyzer errors
// see https://github.com/rust-analyzer/rust-analyzer/issues/11425
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "./ast/grammar.pest"]
pub struct IdentParser;
