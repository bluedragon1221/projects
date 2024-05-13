use super::ast::*;

use pest::{
    iterators::{Pair, Pairs},
    Parser,
};

#[derive(pest_derive::Parser)]
#[grammar = "grammar.pest"]
pub struct LangParser;

#[derive(Debug, thiserror::Error)]
pub enum ParserError {
    #[error("You passed wrong arguments to the function")]
    NumOfArgs(usize),
    #[error("Not expecting that rule")]
    UnexpectedRule { rule: Rule, parse_step: String },
    #[error("You didn't end the program")]
    UnclosedProgram,
    #[error("An error when lexing grammar")]
    PestyError(#[from] pest::error::Error<Rule>),
}

pub type Result<T> = std::result::Result<T, ParserError>;

trait ParseInto<T> {
    fn tf_parse(pair: &Pair<Rule>) -> Result<T>;
}

impl ParseInto<Operator> for Operator {
    fn tf_parse(pair: &Pair<Rule>) -> Result<Operator> {
        match pair.as_rule() {
            Rule::And => Ok(Operator::And),
            Rule::Or => Ok(Operator::Or),
            r => Err(ParserError::UnexpectedRule {
                rule: r,
                parse_step: "Operator".to_string(),
            }),
        }
    }
}

impl ParseInto<Box<Node>> for bool {
    fn tf_parse(pair: &Pair<Rule>) -> Result<Box<Node>> {
        match pair.as_rule() {
            Rule::True => Ok(Box::new(Node::Value(true))),
            Rule::False => Ok(Box::new(Node::Value(false))),
            Rule::Expr => Expr::tf_parse(pair),
            r => Err(ParserError::UnexpectedRule {
                rule: r,
                parse_step: "Value".to_string(),
            }),
        }
    }
}

impl ParseInto<Box<Node>> for Expr {
    fn tf_parse(expr: &Pair<Rule>) -> Result<Box<Node>> {
        let inner: Vec<Pair<Rule>> = expr.clone().into_inner().collect(); // must be a vector, because you can't index Pairs<Rule>

        if inner.len() != 3 {
            return Err(ParserError::NumOfArgs(inner.len()));
        }

        Ok(Box::new(Node::Expr(Expr {
            op: Operator::tf_parse(&inner[0])?,
            lhs: bool::tf_parse(&inner[1])?,
            rhs: bool::tf_parse(&inner[2])?,
        })))
    }
}

fn pesty_parse(program: &str) -> Result<Pairs<Rule>> {
    Ok(LangParser::parse(Rule::Program, program)?)
}

pub fn pesty_fmt(program: &str) -> Result<String> {
    Ok(format!("{:#?}", pesty_parse(program)?))
}

pub fn parse_program(program: &str) -> Result<Program> {
    let pairs = pesty_parse(program)?;
    let mut ast: Vec<Node> = Vec::new();

    let iterator = pairs.into_iter().collect::<Vec<_>>()[0]
        .clone()
        .into_inner();

    for pair in iterator {
        ast.push(match pair.as_rule() {
            Rule::Expr => *Expr::tf_parse(&pair)?, // deref unboxes Node
            Rule::True | Rule::False => *bool::tf_parse(&pair)?,
            Rule::EOI => return Ok(Program::new(ast)),
            a => {
                return Err(ParserError::UnexpectedRule {
                    rule: a,
                    parse_step: "Program".to_string(),
                })
            }
        });
    }

    Err(ParserError::UnclosedProgram)
}
