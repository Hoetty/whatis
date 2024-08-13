#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Identifier(String),
    Value(f64),

    Plus,
    Minus,
    Star,
    Slash,
    Percent,

    LeftParen,
    RightParen,
    Comma,

    Eof,
    Invalid
}