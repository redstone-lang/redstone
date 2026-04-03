use logos::Logos;

#[derive(Logos, Debug, PartialEq, Clone)]
#[logos(skip r"[ \t\r\n]+")]
#[logos(skip(r"//[^\n]*", allow_greedy = true))]
pub enum Token {
    #[token("fn")]
    Fn,
    #[token("let")]
    Let,
    #[token("return")]
    Return,
    #[token("print")]
    Print,

    #[regex(r"[0-9]+", |lex| lex.slice().parse::<i64>().ok())]
    Int(i64),

    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*", |lex| lex.slice().to_string())]
    Ident(String),

    #[token("+")]  Plus,
    #[token("-")]  Minus,
    #[token("*")]  Star,
    #[token("/")]  Slash,
    #[token("=")]  Eq,
    #[token("(")]  LParen,
    #[token(")")]  RParen,
    #[token("{")]  LBrace,
    #[token("}")]  RBrace,
    #[token(";")]  Semi,
    #[token(",")]  Comma,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Spanned {
    pub token: Token,
    pub start: usize,
    pub end: usize,
}

pub fn tokenize(src: &str) -> Result<Vec<Spanned>, LexError> {
    let mut tokens = Vec::new();
    for (result, span) in Token::lexer(src).spanned() {
        match result {
            Ok(token) => tokens.push(Spanned { token, start: span.start, end: span.end }),
            Err(_) => return Err(LexError { pos: span.start }),
        }
    }
    Ok(tokens)
}

#[derive(Debug)]
pub struct LexError {
    pub pos: usize,
}
