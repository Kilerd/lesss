use itertools::Itertools;
use pest_consume::{match_nodes, match_nodes_, Error};

#[derive(Debug)]
pub struct LessRoot {
    pub items: Vec<Item>,
}

#[derive(Debug)]
pub enum Item {
    VariableDel(VariableDel),
    CssBlock(CssBlock),
}

#[derive(Debug)]
pub struct CssBlock {
    pub headers: Vec<CssBlockHeader>,
    pub items: Vec<CssBlockItem>,
}

#[derive(Debug)]
pub enum CssBlockHeader {
    CssIdentifier(CssIdentifier),
    MixinIdentifier(String),
}
#[derive(Debug)]
pub struct CssIdentifier {
    values: Vec<String>
}

#[derive(Debug)]
pub enum CssBlockItem {
    VariableDel(VariableDel),
    CssBlock(CssBlock),
    Mixin(String),
    CssItem(CssItem),
}

#[derive(Debug)]
pub struct CssItem {
    pub name: String,
    pub value: VariableValue,
}

#[derive(Debug)]
pub struct VariableDel {
    pub name: String,
    pub value: VariableValue,
}

#[derive(Debug)]
pub enum VariableValue {
    Expr(VariableExpr),
}

#[derive(Debug)]
pub enum ExprOperator {
    Mul,
    Div,
    Add,
    Sub,
}

#[derive(Debug)]
pub enum VariableExpr {
    Operation(AtExpr, ExprOperator, Box<VariableExpr>),
    Single(AtExpr),
}

#[derive(Debug)]
pub enum AtExpr {
    Operation(TermExpr, ExprOperator, Box<AtExpr>),
    Single(TermExpr),
}

#[derive(Debug)]
pub enum TermExpr {
    VariableName(String),
    SingleValue(String),
}

#[derive(pest_consume::Parser)]
#[grammar = "./parser/less.pest"]
pub struct LessParser;

type Result<T> = std::result::Result<T, Error<Rule>>;
type Node<'i> = pest_consume::Node<'i, Rule, ()>;

#[pest_consume::parser]
impl LessParser {
    fn variable_name(input: Node) -> Result<String> {
        Ok(input.as_str().to_owned())
    }
    fn single_value(input: Node) -> Result<String> {
        Ok(input.as_str().to_owned())
    }
    fn mixin_identifier(input: Node) -> Result<String> {
        Ok(match_nodes!(input.into_children();
            [css_single_identifier(identifier)] => identifier,
        ))
    }
    fn css_single_identifier(input: Node) -> Result<String> {
        Ok(input.as_str().to_owned())
    }
    fn css_identifier(input: Node) -> Result<CssIdentifier> {
        let ids = match_nodes!(input.into_children();
            [css_single_identifier(ids)..] => ids.collect_vec(),
        );
        Ok(CssIdentifier {
            values: ids
        })
    }
    fn css_variable_name(input: Node) -> Result<String> {
        Ok(input.as_str().to_owned())
    }

    fn EOI(_input: Node) -> Result<()> {
        Ok(())
    }
    pub(crate) fn less(input: Node) -> Result<LessRoot> {
        let items = match_nodes!(input.into_children();
            [item(items).., EOI(_)] => items.collect_vec(),
        );
        Ok(LessRoot { items })
    }
    fn item(input: Node) -> Result<Item> {
        Ok(match_nodes!(input.into_children();
            [variable_del(variable_del)] => Item::VariableDel(variable_del),
            [css_block(block)] => Item::CssBlock(block),
        ))
    }
    fn variable_del(input: Node) -> Result<VariableDel> {
        let (name, value): (String, VariableValue) = match_nodes!(input.into_children();
            [variable_name(name), variable_value(value)] => (name, value),
        );
        Ok(VariableDel { name, value })
    }

    fn variable_value(input: Node) -> Result<VariableValue> {
        Ok(match_nodes!(input.into_children();
            [variable_expr(expr)] => VariableValue::Expr(expr),
        ))
    }
    fn variable_expr(input: Node) -> Result<VariableExpr> {
        Ok(match_nodes!(input.into_children();
            [at_expr(lhs), at_expr_op(op), variable_expr(rhs)] => VariableExpr::Operation(lhs, op, Box::new(rhs)),
            [at_expr(lhs)] => VariableExpr::Single(lhs),
        ))
    }
    fn at_expr(input: Node) -> Result<AtExpr> {
        Ok(match_nodes!(input.into_children();
            [term_expr(lhs), term_expr_op(op), at_expr(rhs)] => AtExpr::Operation(lhs, op, Box::new(rhs)),
            [term_expr(lhs)] => AtExpr::Single(lhs),
        ))
    }
    fn at_expr_op(input: Node) -> Result<ExprOperator> {
        Ok(match input.as_str() {
            "*" => ExprOperator::Mul,
            "/" => ExprOperator::Div,
            _ => unreachable!(),
        })
    }
    fn term_expr_op(input: Node) -> Result<ExprOperator> {
        Ok(match input.as_str() {
            "+" => ExprOperator::Mul,
            "-" => ExprOperator::Div,
            _ => unreachable!(),
        })
    }
    fn term_expr(input: Node) -> Result<TermExpr> {
        Ok(match_nodes!(input.into_children();
            [variable_name(name)] => TermExpr::VariableName(name),
            [single_value(value)] => TermExpr::SingleValue(value),
        ))
    }
    fn css_block(input: Node) -> Result<CssBlock> {
        let (headers, items): (Vec<CssBlockHeader>, Vec<CssBlockItem>) = match_nodes!(input.into_children();
            [css_block_headers(headers), css_block_item(items)..] =>(headers, items.collect_vec()),
            [css_block_headers(headers)] =>(headers, vec![]),
        );
        Ok(CssBlock { headers, items })
    }
    fn css_block_headers(input: Node) -> Result<Vec<CssBlockHeader>> {
        Ok(match_nodes!(input.into_children();
            [css_block_header(headers)..] => headers.collect_vec(),
        ))
    }
    fn css_block_header(input: Node) -> Result<CssBlockHeader> {
        Ok(match_nodes!(input.into_children();
            [mixin_identifier(mixin)] => CssBlockHeader::MixinIdentifier(mixin),
            [css_identifier(identifier)] => CssBlockHeader::CssIdentifier(identifier),
        ))
    }
    fn css_block_item(input: Node) -> Result<CssBlockItem> {
        Ok(match_nodes!(input.into_children();
            [variable_del(variable_del)] => CssBlockItem::VariableDel(variable_del),
            [css_block(block)] => CssBlockItem::CssBlock(block),
            [mixin(mixin)] => CssBlockItem::Mixin(mixin),
            [simple_css_item(css_item)] => CssBlockItem::CssItem(css_item)
        ))
    }

    fn mixin(input: Node) -> Result<String> {
        Ok(match_nodes!(input.into_children();
            [mixin_identifier(mixin_identifier)] => mixin_identifier,
        ))
    }
    fn simple_css_item(input: Node) -> Result<CssItem> {
        let (name, value): (String, VariableValue) = match_nodes!(input.into_children();
            [css_variable_name(name), variable_value(value)] => (name, value)
        );
        Ok(CssItem { name, value })
    }
}

#[cfg(test)]
mod test {
    use crate::parser::ast::LessParser;
    use crate::parser::ast::Rule;
    use pest_consume::{Parser, parser};
    use indoc::indoc;

    macro_rules! parse_rule {
        ($rule_type: expr, $rule_fn: tt,   $content: expr) => {
            let inputs = LessParser::parse($rule_type, $content).unwrap();
            let input = inputs.single().unwrap();
            let ret = LessParser::$rule_fn(input);
            println!("{ret:#?}");
            assert!(ret.is_ok());
        };
    }

    #[test]
    fn should_pass_less() {
        parse_rule! {Rule::less, less,  ".a {}"}
        parse_rule! {Rule::less, less, ".a {color:red;}"}

        parse_rule! {Rule::less, less, indoc!(r##"
            #menu a {
              color: #111;
              .bordered();
            }

            .post a {
              color: red;
              .bordered();
            }
        "##)}
    }

    #[test]
    fn should_pass_variable_del() {
        parse_rule! {Rule::variable_del, variable_del, "@width: 10px;"}
        parse_rule! {Rule::variable_del, variable_del, "@height: @width + 10px;"}
    }

    #[test]
    fn should_pass_css_block() {
        parse_rule! {Rule::css_block, css_block, indoc!(r##"
            #header .logo {
              width: 300px;
            }
        "##)}

        parse_rule! {Rule::css_block, css_block, indoc!(r##"
            #header {
              color: black;
              .navigation {
                font-size: 12px;
              }
              .logo {
                width: 300px;
              }
            }
        "##)}
        parse_rule! {Rule::css_block, css_block, indoc!(r##"
            #header, a, p, .a() {}
        "##)}
    }
}
