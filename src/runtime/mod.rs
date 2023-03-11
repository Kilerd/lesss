use std::cell::RefCell;
use std::collections::HashMap;
use std::env::var;
use std::fmt::{Debug, Formatter};
use std::rc::Rc;
use crate::parser::ast::{VariableDel, VariableValue};

pub(crate) mod processable;


pub struct Scope {
    parent: Option<Rc<RefCell<Scope>>>,
    children: Vec<Rc<RefCell<Scope>>>,
    variables: HashMap<String, VariableValue>,
    css: HashMap<String, VariableValue>,
    mixin: Vec<String>
}

impl Debug for Scope {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Scope")
            .field("variables",&self.variables)
            .field("mixin",&self.mixin)
            .field("css",&self.css)
            .field("children",&self.children)
            .finish()
    }
}

impl Scope {
    pub(crate) fn new() -> Self {
        Scope {
            parent: None,
            children: vec![],
            variables: HashMap::new(),
            css: HashMap::new(),
            mixin: Vec::new(),
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
}