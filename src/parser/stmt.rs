use pest::iterators::Pair;
use crate::ast::Stmt;
use super::Rule;
use super::expr::parse_expr;

pub fn parse_stmt(pair: Pair<Rule>) -> Stmt {
    let inner = pair.into_inner().next().unwrap();
    match inner.as_rule() {
        Rule::let_stmt => {
            let mut it = inner.into_inner();
            let name = it.next().unwrap().as_str().to_string();
            let expr = parse_expr(it.next().unwrap());
            Stmt::Let(name, expr)
        }
        Rule::return_stmt => Stmt::Return(parse_expr(inner.into_inner().next().unwrap())),
        Rule::print_stmt => Stmt::Print(parse_expr(inner.into_inner().next().unwrap())),
        Rule::expr_stmt => Stmt::Expr(parse_expr(inner.into_inner().next().unwrap())),
        _ => unreachable!(),
    }
}
