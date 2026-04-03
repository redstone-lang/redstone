use pest::iterators::Pair;
use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "parser/redstone.pest"]
pub struct RedstoneParser;

#[derive(Debug, Clone)]
pub enum Expr {
    Int(i64),
    Var(String),
    BinOp(Box<Expr>, BinOp, Box<Expr>),
    Call(String, Vec<Expr>),
}

#[derive(Debug, Clone)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug, Clone)]
pub enum Stmt {
    Let(String, Expr),
    Return(Expr),
    Print(Expr),
    Expr(Expr),
}

#[derive(Debug, Clone)]
pub struct FnDef {
    pub name: String,
    pub params: Vec<String>,
    pub body: Vec<Stmt>,
}

pub fn parse(src: &str) -> Vec<FnDef> {
    let pairs = RedstoneParser::parse(Rule::program, src)
        .expect("parse error")
        .next()
        .unwrap();
    pairs.into_inner()
        .filter(|p| p.as_rule() == Rule::fn_def)
        .map(parse_fn)
        .collect()
}

fn parse_fn(pair: Pair<Rule>) -> FnDef {
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

fn parse_stmt(pair: Pair<Rule>) -> Stmt {
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

fn parse_expr(pair: Pair<Rule>) -> Expr {
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

fn parse_term(pair: Pair<Rule>) -> Expr {
    let inner = pair.into_inner().next().unwrap();
    match inner.as_rule() {
        Rule::int => Expr::Int(inner.as_str().parse().unwrap()),
        Rule::ident => Expr::Var(inner.as_str().to_string()),
        Rule::call => {
            let mut it = inner.into_inner();
            let name = it.next().unwrap().as_str().to_string();
            let args = it.next()
                .map(|a| a.into_inner().map(parse_expr).collect())
                .unwrap_or_default();
            Expr::Call(name, args)
        }
        Rule::expr => parse_expr(inner),
        _ => unreachable!(),
    }
}
