use std::fs;

use crate::lexer::lexer::Lexer;

mod global;
mod lexer;
mod parser;
mod vec;

fn main() {
    let source = fs::read_to_string("./test/example.vs").unwrap();
    let mut lexer = Lexer::new(source.as_str());

    while let Some(token) = lexer.next_token().unwrap() {
        println!("{:?}", token);
    }
}
