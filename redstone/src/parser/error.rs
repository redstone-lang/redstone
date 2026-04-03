use std::fmt;

#[derive(Debug)]
pub struct ParseError {
    pub message: String,
    pub line: usize,
    pub col: usize,
    pub line_src: String,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "error: {}\n  --> input:{}:{}\n   |\n{:>3}| {}\n   | {}^",
            self.message,
            self.line,
            self.col,
            self.line,
            self.line_src,
            " ".repeat(self.col - 1),
        )
    }
}

pub fn new(src: &str, pos: usize, message: impl Into<String>) -> ParseError {
    let before = &src[..pos.min(src.len())];
    let line = before.chars().filter(|&c| c == '\n').count() + 1;
    let col = before.rfind('\n').map(|i| pos - i - 1).unwrap_or(pos) + 1;
    let line_src = src.lines().nth(line - 1).unwrap_or("").to_string();
    ParseError { message: message.into(), line, col, line_src }
}
