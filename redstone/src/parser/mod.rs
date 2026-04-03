mod lexer;
mod error;
mod cursor;
mod expr;
mod stmt;
mod function;

use crate::ast::Function;
use cursor::Parser;
use error::ParseError;
use lexer::tokenize;

pub fn parse(src: &str) -> Result<Vec<Function>, ParseError> {
    let tokens = tokenize(src).map_err(|e| error::new(src, e.pos, "unexpected character"))?;
    let mut p = Parser::new(src, tokens);
    let mut fns = vec![];
    while !p.is_eof() {
        fns.push(function::parse_fn(&mut p)?);
    }
    Ok(fns)
}
