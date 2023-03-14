use std::cell::RefCell;
use std::collections::HashMap;
use std::env::var;
use std::fmt::{Debug, Formatter};
use std::rc::Rc;
use itertools::Either;
use crate::parser::ast::{CssBlockHeader, CssItem, VariableDel, VariableValue};

pub(crate) mod processable;
pub(crate) mod executable;
pub(crate) mod printable;

type MixinIdentifier = String;


pub struct Scope {
    headers: Vec<CssBlockHeader>,
    parent: Option<Rc<RefCell<Scope>>>,
    children: Vec<Rc<RefCell<Scope>>>,
    variables: HashMap<String, VariableValue>,
    items: Vec<Either<CssItem, MixinIdentifier>>,

    calculated_variables: HashMap<String, String>,
    calculated_css: HashMap<String, String>,

}

impl Debug for Scope {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Scope")
            .field("headers", &self.headers)
            .field("variables", &self.variables)
            .field("items", &self.items)
            .field("children", &self.children)
            .finish()
    }
}

impl Scope {
    pub(crate) fn new() -> Self {
        Scope {
            parent: None,
            headers: Vec::new(),
            children: vec![],
            variables: HashMap::new(),
            items: Vec::new(),
            calculated_variables: Default::default(),
            calculated_css: Default::default(),
        }
    }
    fn new_from_parent(parent: Rc<RefCell<Scope>>) -> Rc<RefCell<Scope>> {
        let mut child = Scope::new();
        child.parent = Some(parent.clone());
        let rc = Rc::new(RefCell::new(child));
        let mut parent_mut = parent.borrow_mut();
        parent_mut.children.push(rc.clone());
        rc
    }

    fn insert_variable(&mut self, variable: VariableDel) {
        let VariableDel { name, value } = variable;
        self.variables.insert(name, value);
    }
    fn get_variable(&self, key: &str) -> Option<VariableValue> {
        let self_variable = self.variables.get(key).map(|it|it.clone());
        if self_variable.is_some() {
            return self_variable;
        }
        let option = self.parent.as_ref();
        option
            .map(|it| it.borrow())
            .and_then(|parent| parent.get_variable(key))

    }
}