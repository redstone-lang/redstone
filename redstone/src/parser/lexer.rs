use logos::Logos;

#[derive(Logos, Debug, PartialEq, Clone)]
#[logos(skip r"[ \t\r\n]+")]
#[logos(skip(r"//[^\n]*", allow_greedy = true))]
pub enum Token {
    #[token("fn")]    Fn,
    #[token("let")]   Let,
    #[token("while")] While,
    #[token("if")]    If,
    #[token("else")]  Else,
    #[token("return")] Return,
    #[token("print")] Print,
    #[token("true")]  True,
    #[token("false")] False,

    // Type keywords
    #[token("i8")]    TyI8,
    #[token("i16")]   TyI16,
    #[token("i32")]   TyI32,
    #[token("i64")]   TyI64,
    #[token("i128")]  TyI128,
    #[token("isize")] TyIsize,
    #[token("u8")]    TyU8,
    #[token("u16")]   TyU16,
    #[token("u32")]   TyU32,
    #[token("u64")]   TyU64,
    #[token("u128")]  TyU128,
    #[token("usize")] TyUsize,
    #[token("f32")]   TyF32,
    #[token("f64")]   TyF64,
    #[token("bool")]  TyBool,
    #[token("char")]  TyChar,

    // Float must come before Int so "1.0" isn't tokenised as Int(1) + Dot + Int(0)
    #[regex(r"[0-9]+\.[0-9]+", |lex| lex.slice().parse::<f64>().ok())]
    Float(f64),

    #[regex(r"[0-9]+", |lex| lex.slice().parse::<i64>().ok())]
    Int(i64),

    // Char literal: 'x' or unicode escape '\u{XXXX}'
    #[regex(r"'\\u\{[0-9a-fA-F]+\}'", parse_unicode_escape)]
    #[regex(r"'[^'\\]'", |lex| {
        let s = lex.slice();
        s[1..s.len()-1].chars().next()
    })]
    Char(char),

    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*", |lex| lex.slice().to_string())]
    Ident(String),

    #[token("+")]   Plus,
    #[token("-")]   Minus,
    #[token("*")]   Star,
    #[token("/")]   Slash,
    #[token("==")]  EqEq,
    #[token("!=")]  Ne,
    #[token("<=")]  Le,
    #[token(">=")]  Ge,
    #[token("<")]   Lt,
    #[token(">")]   Gt,
    #[token("+=")]  PlusEq,
    #[token("-=")]  MinusEq,
    #[token("*=")]  StarEq,
    #[token("/=")]  SlashEq,
    #[token("=")]   Eq,
    #[token("(")]   LParen,
    #[token(")")]   RParen,
    #[token("{")]   LBrace,
    #[token("}")]   RBrace,
    #[token(";")]   Semi,
    #[token(",")]   Comma,
    #[token(":")]   Colon,
    #[token("->")]  Arrow,
}

fn parse_unicode_escape(lex: &mut logos::Lexer<Token>) -> Option<char> {
    let s = lex.slice();
    // format: '\u{XXXX}'
    let hex = &s[4..s.len() - 2];
    let code = u32::from_str_radix(hex, 16).ok()?;
    char::from_u32(code)
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
