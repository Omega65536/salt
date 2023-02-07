mod token;
mod lexer;

use std::fs;
use crate::lexer::Lexer;

fn main() {
    let program = fs::read_to_string("example.salt").expect("Unable to read file!");
    let mut lexer = Lexer::new(program.chars());
    let tokens = lexer.lex();
    println!("{:?}", tokens);
}
