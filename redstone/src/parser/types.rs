use crate::ast::Type;
use crate::parser::cursor::Parser;
use crate::parser::error::ParseError;
use crate::parser::lexer::Token;

pub fn parse_type(p: &mut Parser) -> Result<Type, ParseError> {
    let pos = p.current_pos();
    match p.peek().cloned() {
        Some(Token::TyI8)    => { p.bump(); Ok(Type::I8) }
        Some(Token::TyI16)   => { p.bump(); Ok(Type::I16) }
        Some(Token::TyI32)   => { p.bump(); Ok(Type::I32) }
        Some(Token::TyI64)   => { p.bump(); Ok(Type::I64) }
        Some(Token::TyI128)  => { p.bump(); Ok(Type::I128) }
        Some(Token::TyIsize) => { p.bump(); Ok(Type::Isize) }
        Some(Token::TyU8)    => { p.bump(); Ok(Type::U8) }
        Some(Token::TyU16)   => { p.bump(); Ok(Type::U16) }
        Some(Token::TyU32)   => { p.bump(); Ok(Type::U32) }
        Some(Token::TyU64)   => { p.bump(); Ok(Type::U64) }
        Some(Token::TyU128)  => { p.bump(); Ok(Type::U128) }
        Some(Token::TyUsize) => { p.bump(); Ok(Type::Usize) }
        Some(Token::TyF32)   => { p.bump(); Ok(Type::F32) }
        Some(Token::TyF64)   => { p.bump(); Ok(Type::F64) }
        Some(Token::TyBool)  => { p.bump(); Ok(Type::Bool) }
        Some(Token::TyChar)  => { p.bump(); Ok(Type::Char) }
        Some(Token::LParen)  => {
            p.bump();
            p.expect(&Token::RParen)?;
            Ok(Type::Unit)
        }
        _ => {
            let found = match p.peek() {
                Some(t) => format!("`{t:?}`"),
                None => "end of file".to_string(),
            };
            Err(p.error_at(pos, format!("expected type, found {found}")))
        }
    }
}
