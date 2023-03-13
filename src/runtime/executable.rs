use std::cell::RefCell;
use std::rc::Rc;
use crate::parser::ast::{AtExpr, ExprOperator, TermExpr, VariableExpr, VariableValue};
use crate::Scope;

trait Executable {
    type Output;
    fn execute(&self, scope: Rc<RefCell<Scope>>) -> Result<Self::Output, ()>;
}


impl Scope {
    fn execute(&mut self) -> Result<(), ()> {
        todo!()
    }
}

impl Executable for VariableValue {
    type Output = String;
    fn execute(&self, scope: Rc<RefCell<Scope>>) -> Result<Self::Output, ()> {
        match self {
            VariableValue::Expr(expr) => {
                expr.execute(scope)
            }
        }
    }
}

impl Executable for VariableExpr {
    type Output = String;
    fn execute(&self, scope: Rc<RefCell<Scope>>) -> Result<Self::Output, ()> {
        match self {
            VariableExpr::Operation(lhs, op, rhs) => {
                let lhs_result = lhs.execute(scope.clone());
                let rhs_result = rhs.execute(scope);
                match op {
                    ExprOperator::Add => {
                        // todo implement operator
                        lhs_result
                    }
                    ExprOperator::Sub => {
                        // todo implement operator
                        lhs_result
                    },
                    _=> unreachable!()
                }
            }
            VariableExpr::Single(single) => {
                single.execute(scope)
            }
        }
    }
}

impl Executable for AtExpr {
    type Output = String;
    fn execute(&self, scope: Rc<RefCell<Scope>>) -> Result<Self::Output, ()> {
        match self {
            AtExpr::Operation(lhs, op, rhs) => {
                let lhs_result = lhs.execute(scope.clone());
                let rhs_result = rhs.execute(scope);
                match op {
                    ExprOperator::Add => {
                        // todo implement operator
                        lhs_result
                    }
                    ExprOperator::Sub => {
                        // todo implement operator
                        lhs_result
                    },
                    _=> unreachable!()
                }
            }
            AtExpr::Single(single) => {
                single.execute(scope)
            }
        }
    }
}

impl Executable for TermExpr {
    type Output = String;
    fn execute(&self, scope: Rc<RefCell<Scope>>) -> Result<Self::Output, ()> {
        match self {
            TermExpr::VariableName(variable_name) => {
                let x = scope.clone().borrow();
                x.get_variable(variable_name).expect("cannot get variable").execute(scope)
            }
            TermExpr::SingleValue(simple_value) => { Ok(simple_value.to_owned()) }
        }.to_owned()
    }
}