use std::cell::RefCell;
use std::rc::Rc;
use crate::{Processable, Scope};

trait Processor {
    fn process(&mut self, scope: Rc<RefCell<Scope>>);
}

