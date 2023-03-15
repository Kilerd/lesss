use itertools::Itertools;
use crate::parser::ast::{CssBlockHeader, CssIdentifier};

use crate::runtime::Scope;

pub trait ScopePrintable {
    fn print(&self, parent_headers: &[CssBlockHeader]) -> Vec<String>;
}

pub trait Printable {
    fn print(&self) -> Vec<String>;
}


fn merge_css_header(parent: &[CssBlockHeader], child: &[CssBlockHeader]) -> Vec<CssBlockHeader> {
    if parent.is_empty() { return child.to_vec(); };
    if child.is_empty() { return parent.to_vec(); };
    parent
        .iter()
        .cartesian_product(child)
        .map(|(parent_item, child_item)| {
            match (parent_item, child_item) {
                (CssBlockHeader::CssIdentifier(parent_css), CssBlockHeader::CssIdentifier(child_css)) => {
                    let mut merged_item = parent_css.values.clone();
                    merged_item.extend(child_css.values.clone());
                    CssBlockHeader::CssIdentifier(CssIdentifier {
                        values: merged_item,
                    })
                }
                _ => unreachable!()
            }
        })
        .collect_vec()
}


impl ScopePrintable for Scope {
    fn print(&self, parent_headers: &[CssBlockHeader]) -> Vec<String> {
        let mut buffer = Vec::new();
        let headers = merge_css_header(parent_headers, &self.headers);
        for header in &headers {
            buffer.extend(Printable::print(header));
            buffer.push("{".to_string());
            for (css_key, css_value) in &self.calculated_css {
                buffer.push(format!("  {}: {};", css_key, css_value));
            }
            buffer.push("}".to_string());
        }

        for child in &self.children {
            let ref_mut = child.borrow_mut();
            buffer.extend(ref_mut.print(&headers));
        }

        return buffer;
    }
}

impl Printable for CssBlockHeader {
    fn print(&self) -> Vec<String> {
        match self {
            CssBlockHeader::CssIdentifier(ident) => { vec![ident.values.join(" ")] }
            CssBlockHeader::MixinIdentifier(_) => { todo!() }
        }
    }
}