use std::{error::Error, fmt::Display};

use expression::Expression;

use crate::lexer::token::Token;

pub mod expression;

struct Parser {
    tokens: Vec<Token>,
    position: usize,
    had_error: bool
}

#[derive(Debug)]
pub struct ParseError {
    pub expression: Expression
}
impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Parser Error occured")?;

        Ok(())
    }
}

impl Error for ParseError { }


pub fn parse(tokens: Vec<Token>) -> Result<Expression, ParseError> {
    let mut parser = Parser { tokens, position: 0, had_error: false };
    
    let expression = parser.expression();

    parser.expect(Token::Eof);

    if parser.had_error {
        return Err(ParseError { expression });
    }

    Ok(expression)
}

impl Parser {

    pub fn expression(&mut self) -> Expression {
        self.group()
    }

    pub fn group(&mut self) -> Expression {
        let must_close = self.matches(Token::LeftParen);

        let expression = self.plus_minus();

        if must_close {
            self.expect(Token::RightParen);
        }

        expression
    }

    pub fn plus_minus(&mut self) -> Expression {
        let mut expression = self.mult_div_rem();

        loop {
            let token = self.current_token();
            if !(token == Token::Minus || token == Token::Plus) {
                break;
            }

            self.consume();

            expression = Expression::Binary(Box::new(expression), token, Box::new(self.mult_div_rem()));
        }

        expression
    }

    pub fn mult_div_rem(&mut self) -> Expression {
        let mut expression = self.unary();

        loop {
            let token = self.current_token();
            if !(token == Token::Star || token == Token::Slash || token == Token::Percent) {
                break;
            }

            self.consume();

            expression = Expression::Binary(Box::new(expression), token, Box::new(self.unary()));
        }

        expression
    }

    pub fn unary(&mut self) -> Expression {
        let t = self.current_token();

        if t == Token::Plus || t == Token::Minus {
            self.consume();
            return Expression::Unary(t, Box::new(self.unary()));
        }

        self.call()
    }

    pub fn call(&mut self) -> Expression {
        let expression = self.value();

        if !self.matches(Token::LeftParen) {
            return expression;
        }

        let mut params: Vec<Expression> = Vec::new();

        loop {
            if self.current_token() == Token::RightParen {
                break;
            }

            params.push(self.expression());

            if !self.matches(Token::Comma) {
                self.expect(Token::RightParen);
                break;
            }
        }

        match expression {
            Expression::Identifier(s) => Expression::Call(s, params),
            e => {
                println!("Invalid calle {:?}", e);

                Expression::Invalid
            }
        }

        
    }

    pub fn value(&mut self) -> Expression {
        let token = self.consume();
        match token {
            Token::Identifier(s) => Expression::Identifier(s),
            Token::Value(v) => Expression::Value(v),
            _ => {
                println!("Can't parse Token {:?}", token);
                self.had_error = true;

                Expression::Invalid
            }
        }
    }

    pub fn is_at_end(&self) -> bool {
        self.position >= self.tokens.len()
    }

    pub fn current_token(&self) -> Token {
        if self.is_at_end() {
            return Token::Eof;
        }

        return self.tokens[self.position].clone();
    }

    pub fn consume(&mut self) -> Token {
        let token = self.current_token();
        self.position += 1;

        token
    }

    pub fn matches(&mut self, token: Token) -> bool {
        if self.current_token() == token {
            self.consume();
            
            true
        } else {
            false
        }
    }

    pub fn expect(&mut self, token: Token) {
        let c = self.current_token();
        if !self.matches(token.clone()) {
            println!("Expected {:?} but got {:?}", token, c);
        }
    }

}