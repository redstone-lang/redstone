use pest::iterators::Pair;
use crate::ast::{BinOp, Expr};
use super::Rule;
use super::term::parse_term;

pub fn parse_expr(pair: Pair<Rule>) -> Expr {
    let mut inner = pair.into_inner();
    let mut lhs = parse_term(inner.next().unwrap());
    while let Some(op_pair) = inner.next() {
        let op = match op_pair.as_str() {
            "+" => BinOp::Add,
            "-" => BinOp::Sub,
            "*" => BinOp::Mul,
            "/" => BinOp::Div,
            _ => unreachable!(),
        };
        let rhs = parse_term(inner.next().unwrap());
        lhs = Expr::BinOp(Box::new(lhs), op, Box::new(rhs));
    }
    lhs
}
