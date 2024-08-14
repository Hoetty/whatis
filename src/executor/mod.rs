use environment::Environment;

use crate::{lexer::token::Token, parser::expression::Expression};

pub mod environment;
pub struct Executor {
    environment: Environment,
}

pub fn evaluate_expression(expression: Expression) -> f64 {
    let mut executor = Executor { environment: Environment::default() };
    
    executor.evaluate(expression)
}

impl Executor {
    pub fn evaluate(&mut self, expression: Expression) -> f64 {
        match expression {
            Expression::Unary(t, b) => self.unary(t, *b),
            Expression::Binary(l, t, r) => self.binary(*l, t, *r),
            Expression::Group(e) => self.group(*e),
            Expression::Call(i, v) => self.call(i, v),
            Expression::Identifier(s) => self.identifier(s),
            Expression::Value(v) => self.value(v),
            Expression::Invalid => {
                println!("Invalid Expression Evaluated");

                f64::NAN
            },
        }
    }

    pub fn call(&mut self, name: String, args: Vec<Expression>) -> f64 {
        let values = args.iter().map(|e| self.evaluate(e.clone())).collect();

        self.environment.call(&name, values)
    }

    pub fn identifier(&mut self, name: String) -> f64 {
        self.environment.get(&name)
    }

    pub fn unary(&mut self, token: Token, expression: Expression) -> f64 {
        let value = self.evaluate(expression);
        
        match token {
            Token::Plus => value,
            Token::Minus => -value,
            _ => {
                println!("Invalid Unary Operator {:?}", token);

                f64::NAN
            }
        }
    }

    pub fn binary(&mut self, left: Expression, token: Token, right: Expression) -> f64 {
        let a = self.evaluate(left);
        let b = self.evaluate(right);
        
        match token {
            Token::Plus => a + b,
            Token::Minus => a - b,
            Token::Star => a * b,
            Token::Slash => a / b,
            Token::Percent => a % b,
            _ => {
                println!("Invalid Binary Operator {:?}", token);

                f64::NAN
            }
        }
    }

    pub fn group(&mut self, group: Expression) -> f64 {
        return self.evaluate(group);
    }

    pub fn value(&mut self, value: f64) -> f64 {
        return value;
    }
}