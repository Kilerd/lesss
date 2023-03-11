use crate::parser::ast::{AtExpr, CssBlock, CssBlockItem, CssItem, LessRoot, TermExpr, VariableDel, VariableExpr};
use crate::runtime::Scope;

pub trait Processable {
    fn process(self, scope: &mut Scope) ->Result<(), ()>;
}


impl Processable for LessRoot {

}

impl Processable for Item {

}

impl Processable for VariableDel {

}

impl Processable for CssBlock {

}

impl Processable for CssBlockItem {

}

impl Processable for CssItem {

}

impl Processable for VariableExpr {

}

impl Processable for AtExpr {

}
impl Processable for TermExpr {

}
