extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::error::Error;
use pest::iterators::Pair;
use pest::Parser;

#[derive(Parser)]
#[grammar = "conway.pest"]
struct ConwayParser;

#[derive(PartialEq, Debug, Clone)]
pub enum AstNode {
    Str(String),
}

pub fn string_parser(string_node: Pair<Rule>) -> AstNode {
    let mut string = String::new();
    println!("{:?}",&string_node);
    for node in string_node.clone().into_inner() {
        match node.as_rule() {
            Rule::Str => string = String::from(node.to_string()),
            _ => unreachable!()
        }
    }
    AstNode::Str(string)
}
pub fn build_ast_from_expr(pair: pest::iterators::Pair<Rule>) -> AstNode {
    match pair.as_rule() {
        Rule::Str => string_parser(pair),
        unknown => panic!("Unknown expr: {:?}", unknown),
    }
}

pub fn parse(source: &str) -> Result<Vec<AstNode>, Error<Rule>> {
    let mut ast = vec![];

    let pairs = ConwayParser::parse(Rule::program, source)?;
    for pair in pairs {
        match pair.as_rule() {
            Rule::Str => {
                ast.push(build_ast_from_expr(pair));
            }
            _ => {}
        }
    }

    Ok(ast)
}

fn main() {
    let astnode = parse("'Hello world'\n").expect("unsuccessful parse");
    //println!("{:?}", &astnode);
}