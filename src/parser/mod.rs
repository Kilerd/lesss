
use crate::parser::ast::{LessParser, LessRoot, Rule};
use pest_consume::{Parser};
pub mod ast;

pub fn parse(content: &str) -> LessRoot {
    let inputs = LessParser::parse(Rule::less, content).unwrap();
    let input = inputs.single().unwrap();
    let ret = LessParser::less(input).unwrap();
    ret
}