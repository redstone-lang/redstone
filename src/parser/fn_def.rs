use pest::iterators::Pair;
use crate::ast::FnDef;
use super::Rule;
use super::stmt::parse_stmt;

pub fn parse_fn(pair: Pair<Rule>) -> FnDef {
    let mut inner = pair.into_inner();
    let name = inner.next().unwrap().as_str().to_string();
    let mut params = vec![];
    let mut body = vec![];
    for p in inner {
        match p.as_rule() {
            Rule::param_list => params = p.into_inner().map(|i| i.as_str().to_string()).collect(),
            Rule::block => body = p.into_inner().map(parse_stmt).collect(),
            _ => {}
        }
    }
    FnDef { name, params, body }
}
