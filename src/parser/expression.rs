use crate::lexer::token::Token;

#[derive(Debug, Clone)]
pub enum Expression {
    Unary(Token, Box<Expression>),
    Binary(Box<Expression>, Token, Box<Expression>),
    Group(Box<Expression>),

    Call(Box<Expression>, Vec<Expression>),

    Identifier(Token),
    Value(Token),

    Invalid,
}