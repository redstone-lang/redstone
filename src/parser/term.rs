use pest::iterators::Pair;
use crate::ast::Expr;
use super::Rule;
use super::expr::parse_expr;

pub fn parse_term(pair: Pair<Rule>) -> Expr {
    let inner = pair.into_inner().next().unwrap();
    match inner.as_rule() {
        Rule::int => Expr::Int(inner.as_str().parse().unwrap()),
        Rule::ident => Expr::Var(inner.as_str().to_string()),
        Rule::call => {
            let mut it = inner.into_inner();
            let name = it.next().unwrap().as_str().to_string();
            let args = it
                .next()
                .map(|a| a.into_inner().map(parse_expr).collect())
                .unwrap_or_default();
            Expr::Call(name, args)
        }
        Rule::expr => parse_expr(inner),
        _ => unreachable!(),
    }
}
