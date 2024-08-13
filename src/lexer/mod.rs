use std::{error::Error, fmt::Display};

use token::Token;

pub mod token;

pub struct Lexer<'a> {
    expression: &'a str,
    position: usize,
    had_error: bool,
}

#[derive(Debug)]
pub struct LexError {
    pub tokens: Vec<Token>
}
impl Display for LexError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Lexical Error occured")?;

        Ok(())
    }
}

impl Error for LexError { }

pub fn lex(expression: &str) -> Result<Vec<Token>, LexError> {
    let mut tokens = Vec::new();
    let mut lexer = Lexer {expression, position: 0, had_error: false};

    while !lexer.is_at_end() {
        let c = lexer.current_char();

        let token: Token = match c {
            _ if c.is_whitespace() => {
                lexer.consume();
                continue;
            },

            _ if c.is_alphabetic() => lexer.word(),
            _ if c.is_numeric() => lexer.value(),
            _ => lexer.operator()
        };

        tokens.push(token);
    }

    tokens.push(Token::Eof);

    if lexer.had_error {
        return Err(LexError { tokens })
    }

    return Ok(tokens);
}

impl <'a> Lexer<'a> {

    pub fn operator(&mut self) -> Token {
        return match self.consume() {
            '+' => Token::Plus,
            '-' => Token::Minus,
            '*' => Token::Star,
            '/' => Token::Slash,
            '%' => Token::Percent,

            '(' => Token::LeftParen,
            ')' => Token::RightParen,

            v => {
                self.had_error = true;
                println!("Invalid Operator {}", v);

                Token::Invalid
            }
        }
    }

    pub fn value(&mut self) -> Token {
        let mut value_str = String::new();

        while self.is_for_number() {
            match self.consume() {
                c if c.is_numeric() || c == '.' => value_str.push(c),
                _ => { }
            }
        }

        let value: f64 = value_str.parse().expect(&format!("Parsing shouldn't fail: {}", value_str));
        return Token::Value(value);
    }

    pub fn word(&mut self) -> Token {
        let mut word = String::new();

        while self.is_for_word() {
            word.push(self.consume());
        }

        return match word {
            // Add keywords here
            w => Token::Identifier(w)
        };
    }

    pub fn current_char(&self) -> char {
        self.expression.chars().nth(self.position).unwrap_or_else(|| '\0')
    }

    pub fn consume(&mut self) -> char {
        let c = self.current_char();
        self.position += 1;

        c
    }

    pub fn is_at_end(&self) -> bool {
        self.position >= self.expression.len()
    }

    pub fn is_for_number(&self) -> bool {
        if self.is_at_end() {
            return false;
        }

        let c = self.current_char();

        c.is_numeric() || c == '.' || c == '_'
    }

    pub fn is_for_word(&self) -> bool {
        if self.is_at_end() {
            return false;
        }

        let c = self.current_char();

        c.is_alphanumeric() || c == '_'
    }
}