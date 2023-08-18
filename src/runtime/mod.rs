use std::cell::{Ref, RefCell};
use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use std::rc::{Rc};
use indexmap::IndexMap;
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
    variables: IndexMap<String, VariableValue>,
    items: Vec<Either<CssItem, MixinIdentifier>>,

    calculated_css: IndexMap<String, String>,

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
            variables: IndexMap::new(),
            items: Vec::new(),
            calculated_css: Default::default(),
        }
    }
    fn new_from_parent(parent: Rc<RefCell<Scope>>) -> Rc<RefCell<Scope>> {
        let mut child = Scope::new();
        child.parent = Some(Rc::clone(&parent));
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
        let self_variable = self.variables.get(key).map(|it| it.clone());
        if self_variable.is_some() {
            return self_variable;
        }
        let option = self.parent.as_ref();
        option
            .map(|it| it.borrow())
            .and_then(|parent| parent.get_variable(key))
    }
    fn find_mixin(&self, name: &MixinIdentifier) -> Option<Rc<RefCell<Scope>>> {
        for x in &self.children {
            let x1 = x.borrow();
            if x1.match_mixin(name) {
                drop(x1);
                return Some(x.clone());
            }
        }
        if let Some(parent) = &self.parent {
            parent.borrow().find_mixin(name)
        } else {
            None
        }
    }
    fn match_mixin(&self, name: &MixinIdentifier) -> bool {
        self.headers.iter().any(|header| match header {
            CssBlockHeader::CssIdentifier(css_ident) => { css_ident.values.iter().any(|ident| ident.eq(name)) }
            CssBlockHeader::MixinIdentifier(my) => { my.eq(name) }
        })
    }
}