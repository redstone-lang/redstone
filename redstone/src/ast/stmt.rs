use crate::ast::Expr;

#[derive(Debug, Clone)]
pub enum Stmt {
    Let(String, Expr),
    Return(Expr),
    Print(Expr),
    Expr(Expr),
}
