use std::cell::RefCell;
use std::rc::Rc;
use itertools::Either;
use crate::parser::ast::{ CssBlock, CssBlockItem, CssItem, Item, LessRoot, VariableDel};
use crate::runtime::Scope;

pub trait Processable {
    fn process(self, scope: Rc<RefCell<Scope>>) ->Result<(), ()>;
}


impl Processable for LessRoot {
    fn process(self, scope: Rc<RefCell<Scope>>) -> Result<(), ()> {
        println!("process less root");
        for item in self.items {
            item.process(scope.clone())?
        }
        Ok(())
    }
}

impl Processable for Item {
    fn process(self, scope: Rc<RefCell<Scope>>) -> Result<(), ()> {
        println!("processable : {:?}", &self);
        match  self{
            Item::VariableDel(variable_del) => variable_del.process(scope)?,
            Item::CssBlock(block) => block.process(scope)?
        }
        Ok(())
    }
}

impl Processable for VariableDel {
    fn process(self, scope: Rc<RefCell<Scope>>) -> Result<(), ()> {
        println!("processable : {:?}", &self);
        let mut ref_mut = scope.borrow_mut();
        ref_mut
            .insert_variable(self);
        Ok(())
    }
}

impl Processable for CssBlock {
    fn process(self, scope: Rc<RefCell<Scope>>) -> Result<(), ()> {
        println!("processable : {:?}", &self);
        let css_scope = Scope::new_from_parent(scope);

        for item in self.items {
            item.process(css_scope.clone())?;
        }
        let mut ref_mut = css_scope.borrow_mut();
        ref_mut.headers.extend(self.headers);
        Ok(())
    }
}

impl Processable for CssBlockItem {
    fn process(self, scope: Rc<RefCell<Scope>>) -> Result<(), ()> {
        println!("processable : {:?}", &self);
        match self {
            CssBlockItem::CssItem(css_item) => css_item.process(scope)?,
            CssBlockItem::CssBlock(block) => block.process(scope)?,
            CssBlockItem::VariableDel(variable_del) => variable_del.process(scope)?,
            CssBlockItem::Mixin(mixin) => {
                let mut ref_mut = scope.borrow_mut();
                ref_mut.items.push(Either::Right(mixin));
            }
        }
        Ok(())
    }
}

impl Processable for CssItem {

    fn process(self, scope: Rc<RefCell<Scope>>) -> Result<(), ()> {
        println!("processable : {:?}", &self);
        let mut ref_mut = scope.borrow_mut();
        ref_mut.items.push(Either::Left(self));
        Ok(())
    }
}
