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
    fn register(mut self) -> Self {
        todo!()
    }
    fn process(mut self) -> Self {
         todo!()
    }
    fn print(&self) -> String {
        todo!()
    }
}