use std::{env, io};

use lexer::lex;
use parser::parse;

pub mod lexer;
pub mod parser;
pub mod executor;

fn main() {
    let args: Vec<String> = env::args()
        .enumerate()
        .filter(|&(i, _)| i != 0)
        .map(|(_, e)| e)
        .collect();

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
    println!("{:?}", tokens);

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
    println!("{:?}", expression);

    if error {
        return;
    }

}