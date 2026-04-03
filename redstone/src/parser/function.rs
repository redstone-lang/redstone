use crate::ast::{Function, Param};
use crate::parser::cursor::Parser;
use crate::parser::error::ParseError;
use crate::parser::lexer::Token;
use crate::parser::stmt::parse_stmt;
use crate::parser::types::parse_type;

pub fn parse_fn(p: &mut Parser) -> Result<Function, ParseError> {
    p.expect(&Token::Fn)?;
    let name = p.expect_ident()?;
    p.expect(&Token::LParen)?;
    let params = parse_param_list(p)?;
    p.expect(&Token::RParen)?;

    let ret = if p.peek() == Some(&Token::Arrow) {
        p.bump();
        Some(parse_type(p)?)
    } else {
        None
    };

    p.expect(&Token::LBrace)?;
    let mut body = vec![];
    while p.peek() != Some(&Token::RBrace) && !p.is_eof() {
        body.push(parse_stmt(p)?);
    }
    p.expect(&Token::RBrace)?;
    Ok(Function { name, params, ret, body })
}

fn parse_param_list(p: &mut Parser) -> Result<Vec<Param>, ParseError> {
    let mut params = vec![];
    if p.peek() == Some(&Token::RParen) {
        return Ok(params);
    }
    params.push(parse_param(p)?);
    while p.peek() == Some(&Token::Comma) {
        p.bump();
        params.push(parse_param(p)?);
    }
    Ok(params)
}

fn parse_param(p: &mut Parser) -> Result<Param, ParseError> {
    let name = p.expect_ident()?;
    let ty = if p.peek() == Some(&Token::Colon) {
        p.bump();
        Some(parse_type(p)?)
    } else {
        None
    };
    Ok(Param { name, ty })
}
