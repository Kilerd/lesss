use pest::Span;
use pest_ast::FromPest;
use from_pest::FromPest;
use crate::parser::Rule;


fn span_into_str(span: Span) -> &str {
    span.as_str()
}
fn span_into_string(span: Span) -> String {
    span.as_str().to_owned()
}


#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::less))]
pub struct Less {
    pub items: Vec<Item>,
    eoi: EOI,
}

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::item))]
pub enum Item {
    VariableDel(VariableDel),
    CssBlock(CssBlock),
}

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::css_block))]
pub struct CssBlock {
    header: Vec<CssBlockHeader>,
    items: Vec<CssBlockItem>,
}

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::css_block_header))]
pub enum CssBlockHeader {
    #[pest_ast(inner(rule(css_identifier)))]
    CssIdentifier {
        #[pest_ast(inner(with(span_into_string)))]
        value: String
    },
    #[pest_ast(inner(rule(mixin_identifier)))]
    MixinIdentifier {
        #[pest_ast(inner(with(span_into_string)))]
        value: String
    },
}

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::css_block_item))]
pub enum CssBlockItem {
    VariableDel(VariableDel),
    CssBlock(CssBlock),
}



#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::variable_del))]
pub struct VariableDel {}


#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::EOI))]
struct EOI;


#[cfg(test)]
mod test {
    use pest::Parser;
    use crate::parser::ast::Less;
    use crate::parser::Rule;
    use crate::parser::LessParser;
    use from_pest::FromPest;

    #[test]
    fn should_pass() {
        let mut res = LessParser::parse(Rule::less, ".a {}").unwrap();
        let less = Less::from_pest(&mut res);
        println!("{less:#?}");
    }
}