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
    use std::cell::RefCell;
    use std::rc::Rc;
    use indoc::indoc;
    use itertools::Itertools;
    use crate::Less;
    use crate::runtime::executable::execute_root;
    use crate::runtime::printable::ScopePrintable;
    use crate::runtime::processable::Processable;
    use crate::runtime::Scope;

    #[test]
    fn integration_test() {
        let dir = std::fs::read_dir("examples").unwrap();
        let testcases = dir.into_iter().filter_map(|it| it.ok())
            .filter_map(|it| {
                it.path().file_stem().and_then(|oss| oss.to_str()).map(|s| s.to_owned())
            }).unique().collect_vec();
        for testcase in testcases {
            println!("testing {}", &testcase);
            let less_source = std::fs::read_to_string(format!("examples/{}.less", testcase)).unwrap();
            let css_source = std::fs::read_to_string(format!("examples/{}.css", testcase)).unwrap();
            let less = Less::new(less_source.as_str());
            let result = less.process();
            assert!(result.is_ok());
            let css = Less::new(css_source.as_str());
            let parsed_css = css.process().unwrap();

            assert_eq!(
                result.unwrap(),
                parsed_css,
            );
        }
    }
}