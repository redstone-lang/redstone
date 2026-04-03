#[derive(Debug, Clone)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug, Clone)]
pub enum Expr {
    Int(i64),
    Var(String),
    BinOp(Box<Expr>, BinOp, Box<Expr>),
    Call(String, Vec<Expr>),
}
