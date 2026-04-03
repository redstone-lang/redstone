use crate::parser::error::{self, ParseError};
use crate::parser::lexer::{Spanned, Token};

pub struct Parser<'src> {
    src: &'src str,
    tokens: Vec<Spanned>,
    pos: usize,
}

impl<'src> Parser<'src> {
    pub fn new(src: &'src str, tokens: Vec<Spanned>) -> Self {
        Self { src, tokens, pos: 0 }
    }

    pub fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.pos).map(|s| &s.token)
    }

    pub fn bump(&mut self) -> Option<&Spanned> {
        let s = self.tokens.get(self.pos);
        if s.is_some() { self.pos += 1; }
        s
    }

    pub fn expect(&mut self, expected: &Token) -> Result<&Spanned, ParseError> {
        match self.tokens.get(self.pos) {
            Some(s) if &s.token == expected => { self.pos += 1; Ok(&self.tokens[self.pos - 1]) }
            Some(s) => Err(self.error_at(s.start, format!("expected `{expected:?}`, found `{:?}`", s.token))),
            None => Err(self.error_at(self.src.len(), format!("expected `{expected:?}`, found end of file"))),
        }
    }

    pub fn expect_ident(&mut self) -> Result<String, ParseError> {
        match self.tokens.get(self.pos).cloned() {
            Some(s) => {
                if let Token::Ident(name) = s.token {
                    self.pos += 1;
                    Ok(name)
                } else {
                    Err(self.error_at(s.start, format!("expected identifier, found `{:?}`", s.token)))
                }
            }
            None => Err(self.error_at(self.src.len(), "expected identifier, found end of file")),
        }
    }

    pub fn current_pos(&self) -> usize {
        self.tokens.get(self.pos).map(|s| s.start).unwrap_or(self.src.len())
    }

    pub fn error_at(&self, pos: usize, message: impl Into<String>) -> ParseError {
        error::new(self.src, pos, message)
    }

    pub fn is_eof(&self) -> bool {
        self.pos >= self.tokens.len()
    }
}
