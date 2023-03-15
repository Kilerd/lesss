
use crate::parser::ast::{LessParser, LessRoot, Rule};
use pest_consume::{Parser};
pub mod ast;

pub fn parse(content: &str) -> Result<LessRoot, String> {
    let inputs = LessParser::parse(Rule::less, content).map_err(|e|e.to_string())?;
    let input = inputs.single().map_err(|e|e.to_string())?;
    LessParser::less(input).map_err(|e|e.to_string())
}