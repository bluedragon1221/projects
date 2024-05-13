use crate::ast;
use pest::Parser;
use pest_consume::{Error, Parser as PParser};

#[derive(PParser)]
#[grammar = "grammar.pest"]
pub struct LangParser;

type Node<'i> = pest_consume::Node<'i, Rule, ()>;
type Result<T> = std::result::Result<T, Error<Rule>>;

impl ToString for Rule {
    fn to_string(&self) -> String {
        format!("{:#?}", self)
    }
}

#[allow(non_snake_case)]
impl LangParser {
    fn EOI(_: Node) -> Result<()> {
        Ok(())
    }

    fn Operator(input: Node) -> Result<ast::Operator> {
        match input.as_rule() {
            Rule::And => Ok(ast::Operator::And),
            Rule::Or => Ok(ast::Operator::Or),
            r => Err(input.error(r)),
        }
    }

    fn Value(input: Node) -> Result<ast::Node> {
        match input.as_rule() {
            Rule::True => Ok(ast::Node::Value(true)),
            Rule::False => Ok(ast::Node::Value(false)),
            Rule::Expr => Self::Expr(input),
            r => Err(input.error(r)),
        }
    }

    fn Expr(input: Node) -> Result<ast::Node> {
        let inner: pest_consume::Nodes<'_, Rule, ()> = input.into_children();

        let mut vec = Vec::new();
        inner.for_each(|i| vec.push(i));

        if let Some((op, args)) = vec.split_first() {
            Ok(ast::Node::Expr {
                op: Self::Operator(op.clone())?,
                args: args
                    .into_iter()
                    .map(|x| Box::new(Self::Value(x.clone()).unwrap()))
                    .collect(),
            })
        } else {
            Err(vec[0].error("asdf"))
        }
    }
}

fn parse_program(input_str: &str) -> Result<ast::Program> {
    let pesty_parsed = LangParser::parse(Rule::Program, input_str)?;

    let mut program = Vec::new();
    for i in pesty_parsed {
        program.push(i)
    }

    Ok(ast::Program::new(program))
}
