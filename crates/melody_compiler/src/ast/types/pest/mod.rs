use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "./ast/types/pest/grammar.pest"]
pub struct IdentParser;
