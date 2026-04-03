use crate::ast::Stmt;
use crate::parser::cursor::Parser;
use crate::parser::error::ParseError;
use crate::parser::expr::parse_expr;
use crate::parser::lexer::Token;

pub fn parse_stmt(p: &mut Parser) -> Result<Stmt, ParseError> {
    match p.peek() {
        Some(Token::Let) => {
            p.bump();
            let name = p.expect_ident()?;
            p.expect(&Token::Eq)?;
            let expr = parse_expr(p)?;
            p.expect(&Token::Semi)?;
            Ok(Stmt::Let(name, expr))
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
