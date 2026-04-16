use crate::ast::{BinOp, Expr, Stmt, Type};
use crate::parser::cursor::Parser;
use crate::parser::error::ParseError;
use crate::parser::expr::{parse_arg_list, parse_expr, parse_multiplicative, peek_additive_or_cmp};
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
                    // Expression statement: may be a call `foo()`, a variable, or a larger
                    // expression `a + b * c` where `a` is already consumed.
                    let lhs = if p.peek() == Some(&Token::LParen) {
                        p.bump();
                        let args = parse_arg_list(p)?;
                        p.expect(&Token::RParen)?;
                        Expr::Call(name, args)
                    } else {
                        Expr::Var(name)
                    };
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
            if p.peek() == Some(&Token::Semi) {
                p.bump();
            }
            Ok(Stmt::Expr(expr))
        }
    }
}

/// Continue parsing an expression whose leading term `lhs` was already consumed.
/// Respects operator precedence: `*`/`/` bind tighter than `+`/`-` and comparisons.
fn parse_expr_with_lhs(p: &mut Parser, lhs: Expr) -> Result<Expr, ParseError> {
    // lhs is at term level — collect any trailing * or / first
    let mut cur = lhs;
    while matches!(p.peek(), Some(Token::Star) | Some(Token::Slash)) {
        let op = if p.peek() == Some(&Token::Star) { BinOp::Mul } else { BinOp::Div };
        p.bump();
        let rhs = crate::parser::expr::parse_term(p)?;
        cur = Expr::BinOp(Box::new(cur), op, Box::new(rhs));
    }
    // cur is now at multiplicative level — handle additive / comparison
    while let Some(op) = peek_additive_or_cmp(p) {
        p.bump();
        let rhs = parse_multiplicative(p)?;
        cur = Expr::BinOp(Box::new(cur), op, Box::new(rhs));
    }
    Ok(cur)
}
