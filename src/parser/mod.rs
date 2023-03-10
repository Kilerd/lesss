#[derive(pest_derive::Parser)]
#[grammar = "./parser/less.pest"]
pub struct LessParser;


pub mod ast;