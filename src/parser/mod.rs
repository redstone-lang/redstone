mod expr;
mod stmt;
mod fn_def;
mod term;

use pest::Parser;
use pest_derive::Parser;
use crate::ast::FnDef;

#[derive(Parser)]
#[grammar = "parser/redstone.pest"]
pub struct RedstoneParser;

pub fn parse(src: &str) -> Vec<FnDef> {
    let pairs = RedstoneParser::parse(Rule::program, src)
        .expect("parse error")
        .next()
        .unwrap();
    pairs
        .into_inner()
        .filter(|p| p.as_rule() == Rule::fn_def)
        .map(fn_def::parse_fn)
        .collect()
}
