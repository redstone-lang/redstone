use crate::ast::{Stmt, Type};
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
        _ => {
            let expr = parse_expr(p)?;
            p.expect(&Token::Semi)?;
            Ok(Stmt::Expr(expr))
        }
    }
}
