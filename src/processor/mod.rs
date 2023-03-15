use std::cell::RefCell;
use std::rc::Rc;
use crate::{Scope};

trait Processor {
    fn process(&mut self, scope: Rc<RefCell<Scope>>);
}

