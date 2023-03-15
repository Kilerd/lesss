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

pub struct Less {
    content: String,
}

impl Less {
    pub fn new(content: &str) -> Self {
        Less {
            content: content.to_owned()
        }
    }
    pub fn process(self) -> Result<String, String> {
        let root = parser::parse(&self.content)?;
        let scope = Rc::new(RefCell::new(Scope::new()));
        root.process(scope.clone()).map_err(|e| format!("cannot process"))?;

        execute_root(scope.clone()).map_err(|e| format!("cannot execute"))?;
        let ref_mut = scope.borrow_mut();
        Ok(ref_mut.print(&[]).join("\n"))
    }
}


#[cfg(test)]
mod test {
    use indoc::indoc;
    use crate::Less;

    #[test]
    fn test() {
        let result1 = Less::new(indoc!(r##"
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
        dbg!(result1);
        let result = Less::new(indoc!(r##"
        a{} b
        "##))
            .process();
        dbg!(result);
    }
}