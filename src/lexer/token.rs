#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Identifier(String),
    Value(f64),

    Plus,
    Minus,
    Star,
    StarStar,
    Slash,
    Percent,

    LeftParen,
    RightParen,
    Comma,

    Eof,
    Invalid
}