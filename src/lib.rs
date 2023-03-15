use std::cell::RefCell;
use std::rc::Rc;
use crate::runtime::executable::execute_root;
use crate::runtime::printable::ScopePrintable;
use crate::runtime::Scope;
use crate::runtime::processable::Processable;

pub mod parser;
pub mod runtime;
pub mod processor;

pub fn parse(content: &str) -> String {
    content.to_owned()
}

struct Less {
    content: String,
}

impl Less {
    fn new(content: &str) -> Self {
        Less {
            content: content.to_owned()
        }
    }
    fn process(self) {
        let root = parser::parse(&self.content);
        let scope = Rc::new(RefCell::new(Scope::new()));
        root.process(scope.clone()).unwrap();

        execute_root(scope.clone()).unwrap();
        let ref_mut = scope.borrow_mut();
        println!("{}", ref_mut.print(&[]));
    }
    fn print(&self) -> String {
        todo!()
    }
}


#[cfg(test)]
mod test {
    use indoc::indoc;
    use crate::Less;

    #[test]
    fn test() {
        Less::new(indoc!(r##"
        @color: blue;
        #header {
          color: black;
          .navigation {
            font-size: 12px;
            color: @color;
          }
          .logo {
            width: 300px;
          }
        }
        "##))
            .process();
    }
}