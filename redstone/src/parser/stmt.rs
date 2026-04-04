use crate::ast::{BinOp, Stmt, Type};
use crate::parser::cursor::Parser;
use crate::parser::error::ParseError;
use crate::parser::expr::parse_expr;
use crate::parser::lexer::Token;
use crate::parser::types::parse_type;

pub fn parse_stmt(p: &mut Parser) -> Result<Stmt, ParseError> {
    match p.peek() {
        Some(Token::Let) => {
            p.bump();
            let name = p.expect_ident()?;
            let ann: Option<Type> = if p.peek() == Some(&Token::Colon) {
                p.bump();
                Some(parse_type(p)?)
            } else {
                None
            };
            p.expect(&Token::Eq)?;
            let expr = parse_expr(p)?;
            p.expect(&Token::Semi)?;
            Ok(Stmt::Let(name, ann, expr))
        }
        Some(Token::While) => {
            p.bump();
            let cond = parse_expr(p)?;
            p.expect(&Token::LBrace)?;
            let mut body = vec![];
            while p.peek() != Some(&Token::RBrace) {
                body.push(parse_stmt(p)?);
            }
            p.expect(&Token::RBrace)?;
            Ok(Stmt::While(cond, body))
        }
        Some(Token::If) => {
            p.bump();
            let cond = parse_expr(p)?;
            p.expect(&Token::LBrace)?;
            let mut then_body = vec![];
            while p.peek() != Some(&Token::RBrace) {
                then_body.push(parse_stmt(p)?);
            }
            p.expect(&Token::RBrace)?;
            let else_body = if p.peek() == Some(&Token::Else) {
                p.bump();
                p.expect(&Token::LBrace)?;
                let mut body = vec![];
                while p.peek() != Some(&Token::RBrace) {
                    body.push(parse_stmt(p)?);
                }
                p.expect(&Token::RBrace)?;
                Some(body)
            } else {
                None
            };
            Ok(Stmt::If(cond, then_body, else_body))
        }
        Some(Token::Return) => {
            p.bump();
            let expr = parse_expr(p)?;
            p.expect(&Token::Semi)?;
            Ok(Stmt::Return(expr))
        }
        Some(Token::Print) => {
            p.bump();
            p.expect(&Token::LParen)?;
            let expr = parse_expr(p)?;
            p.expect(&Token::RParen)?;
            p.expect(&Token::Semi)?;
            Ok(Stmt::Print(expr))
        }
        Some(Token::Ident(_)) => {
            // peek ahead: if next-next is `=` or `+=` etc, it's an assignment
            let name = p.expect_ident()?;
            match p.peek() {
                Some(Token::Eq) => {
                    p.bump();
                    let expr = parse_expr(p)?;
                    p.expect(&Token::Semi)?;
                    Ok(Stmt::Assign(name, expr))
                }
                Some(Token::PlusEq) => { p.bump(); let e = parse_expr(p)?; p.expect(&Token::Semi)?; Ok(Stmt::AssignOp(name, BinOp::Add, e)) }
                Some(Token::MinusEq) => { p.bump(); let e = parse_expr(p)?; p.expect(&Token::Semi)?; Ok(Stmt::AssignOp(name, BinOp::Sub, e)) }
                Some(Token::StarEq) => { p.bump(); let e = parse_expr(p)?; p.expect(&Token::Semi)?; Ok(Stmt::AssignOp(name, BinOp::Mul, e)) }
                Some(Token::SlashEq) => { p.bump(); let e = parse_expr(p)?; p.expect(&Token::Semi)?; Ok(Stmt::AssignOp(name, BinOp::Div, e)) }
                _ => {
                    // not an assignment — treat as expression statement starting with a variable
                    use crate::ast::Expr;
                    let lhs = Expr::Var(name);
                    let expr = parse_expr_with_lhs(p, lhs)?;
                    if p.peek() == Some(&Token::Semi) {
                        p.bump();
                    }
                    Ok(Stmt::Expr(expr))
                }
            }
        }
        _ => {
            let expr = parse_expr(p)?;
            // implicit return: last expr without semicolon
            if p.peek() == Some(&Token::Semi) {
                p.bump();
                Ok(Stmt::Expr(expr))
            } else {
                Ok(Stmt::Expr(expr))
            }
        }
    }
}

fn parse_expr_with_lhs(p: &mut Parser, lhs: crate::ast::Expr) -> Result<crate::ast::Expr, ParseError> {
    use crate::ast::Expr;
    use crate::parser::expr::peek_binop;
    let mut lhs = lhs;
    while let Some(op) = peek_binop(p) {
        p.bump();
        let rhs = crate::parser::expr::parse_term(p)?;
        lhs = Expr::BinOp(Box::new(lhs), op, Box::new(rhs));
    }
    Ok(lhs)
}
