use crate::ast::{Expr, Type};

#[derive(Debug, Clone)]
pub enum Stmt {
    Let(String, Option<Type>, Expr),
    Assign(String, Expr),
    AssignOp(String, crate::ast::BinOp, Expr),
    While(Expr, Vec<Stmt>),
    If(Expr, Vec<Stmt>, Option<Vec<Stmt>>),
    Return(Expr),
    Print(Expr),
    Expr(Expr),
}
