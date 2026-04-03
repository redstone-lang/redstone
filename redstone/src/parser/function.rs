use crate::ast::{Function};
use crate::parser::cursor::Parser;
use crate::parser::error::ParseError;
use crate::parser::lexer::Token;
use crate::parser::stmt::parse_stmt;

pub fn parse_fn(p: &mut Parser) -> Result<Function, ParseError> {
    p.expect(&Token::Fn)?;
    let name = p.expect_ident()?;
    p.expect(&Token::LParen)?;
    let params = parse_param_list(p)?;
    p.expect(&Token::RParen)?;
    p.expect(&Token::LBrace)?;
    let mut body = vec![];
    while p.peek() != Some(&Token::RBrace) && !p.is_eof() {
        body.push(parse_stmt(p)?);
    }
    p.expect(&Token::RBrace)?;
    Ok(Function { name, params, body })
}

fn parse_param_list(p: &mut Parser) -> Result<Vec<String>, ParseError> {
    let mut params = vec![];
    if p.peek() == Some(&Token::RParen) {
        return Ok(params);
    }
    params.push(p.expect_ident()?);
    while p.peek() == Some(&Token::Comma) {
        p.bump();
        params.push(p.expect_ident()?);
    }
    Ok(params)
}
