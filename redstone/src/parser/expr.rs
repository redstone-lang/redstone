use crate::ast::{BinOp, Expr};
use crate::parser::cursor::Parser;
use crate::parser::error::ParseError;
use crate::parser::lexer::Token;

pub fn parse_expr(p: &mut Parser) -> Result<Expr, ParseError> {
    let mut lhs = parse_term(p)?;
    while let Some(op) = peek_binop(p) {
        p.bump();
        let rhs = parse_term(p)?;
        lhs = Expr::BinOp(Box::new(lhs), op, Box::new(rhs));
    }
    Ok(lhs)
}

fn parse_term(p: &mut Parser) -> Result<Expr, ParseError> {
    match p.peek() {
        Some(Token::Int(_)) => {
            let s = p.bump().unwrap();
            if let Token::Int(n) = s.token { Ok(Expr::Int(n)) } else { unreachable!() }
        }
        Some(Token::Float(_)) => {
            let s = p.bump().unwrap();
            if let Token::Float(f) = s.token { Ok(Expr::Float(f)) } else { unreachable!() }
        }
        Some(Token::True) => { p.bump(); Ok(Expr::Bool(true)) }
        Some(Token::False) => { p.bump(); Ok(Expr::Bool(false)) }
        Some(Token::Char(_)) => {
            let s = p.bump().unwrap();
            if let Token::Char(c) = s.token { Ok(Expr::Char(c)) } else { unreachable!() }
        }
        Some(Token::Ident(_)) => {
            let name = p.expect_ident()?;
            if p.peek() == Some(&Token::LParen) {
                p.bump();
                let args = parse_arg_list(p)?;
                p.expect(&Token::RParen)?;
                Ok(Expr::Call(name, args))
            } else {
                Ok(Expr::Var(name))
            }
        }
        Some(Token::LParen) => {
            p.bump();
            if p.peek() == Some(&Token::RParen) {
                p.bump();
                return Ok(Expr::Unit);
            }
            let expr = parse_expr(p)?;
            p.expect(&Token::RParen)?;
            Ok(expr)
        }
        _ => {
            let pos = p.current_pos();
            let found = match p.peek() {
                Some(t) => format!("`{t:?}`"),
                None => "end of file".to_string(),
            };
            Err(p.error_at(pos, format!("expected expression, found {found}")))
        }
    }
}

fn parse_arg_list(p: &mut Parser) -> Result<Vec<Expr>, ParseError> {
    let mut args = vec![];
    if p.peek() == Some(&Token::RParen) {
        return Ok(args);
    }
    args.push(parse_expr(p)?);
    while p.peek() == Some(&Token::Comma) {
        p.bump();
        args.push(parse_expr(p)?);
    }
    Ok(args)
}

fn peek_binop(p: &Parser) -> Option<BinOp> {
    match p.peek() {
        Some(Token::Plus)  => Some(BinOp::Add),
        Some(Token::Minus) => Some(BinOp::Sub),
        Some(Token::Star)  => Some(BinOp::Mul),
        Some(Token::Slash) => Some(BinOp::Div),
        _ => None,
    }
}
