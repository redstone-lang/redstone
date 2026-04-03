use crate::ast::{Expr, Type};

#[derive(Debug, Clone)]
pub enum Stmt {
    Let(String, Option<Type>, Expr),
    Return(Expr),
    Print(Expr),
    Expr(Expr),
}
