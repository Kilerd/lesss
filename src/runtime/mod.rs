use std::collections::HashMap;
use std::env::var;
use crate::parser::ast::{VariableDel, VariableValue};

mod processable;


pub struct Scope {
    variables: HashMap<String, VariableValue>,
}

impl Scope {
    fn new() -> Self {
        Scope {
            variables: HashMap::new()
        }
    }
    fn insert_variable(mut self, variable: VariableDel) {
        let VariableDel { name, value } = variable;
        self.variables.insert(name, value);
    }

}