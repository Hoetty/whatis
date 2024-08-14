use std::{env, io};

use executor::evaluate_expression;
use lexer::{lex, token::Token};
use parser::parse;

pub mod lexer;
pub mod parser;
pub mod executor;

fn main() {
    let args: Vec<String> = env::args().collect::<Vec<_>>()[1..].to_vec();

    if args.len() == 0 {
        repl();
    } else {
        execute(&args.join(" "));
    }
}

fn repl() {
    loop {
        let mut expression = String::new();
        io::stdin().read_line(&mut expression).expect("Couldn't read input!");
        execute(&expression);
    }
}

fn execute(expression: &str) {
    let mut error = false;
    let tokens = match lex(expression) {
        Ok(v) => v,
        Err(e) => {
            error = true;

            e.tokens
        }
    };
    // println!("{:?}", tokens);

    if let [Token::Eof] = tokens[..] {
        return;
    }

    if error {
        return;
    }

    let expression = match parse(tokens) {
        Ok(v) => v,
        Err(e) => {
            error = true;

            e.expression
        }
    };
    // println!("{:?}", expression);

    if error {
        return;
    }

    let value = evaluate_expression(expression);
    println!("{}", value);
}