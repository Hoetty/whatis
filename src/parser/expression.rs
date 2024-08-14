use crate::lexer::token::Token;

#[derive(Debug, Clone)]
pub enum Expression {
    Unary(Token, Box<Expression>),
    Binary(Box<Expression>, Token, Box<Expression>),
    Group(Box<Expression>),

    Call(String, Vec<Expression>),

    Identifier(String),
    Value(f64),

    Invalid,
}