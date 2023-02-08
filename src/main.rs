mod ast;
mod interpreter;
mod lexer;
mod parser;
mod token;
mod value;

use crate::interpreter::Interpeter;
use crate::lexer::Lexer;
use crate::parser::Parser;
use std::fs;

fn main() {
    let program = fs::read_to_string("example.salt").expect("Unable to read file!");

    let mut lexer = Lexer::new(program.chars());
    let tokens = lexer.lex();
    //println!("{:?}", tokens);

    let mut parser = Parser::new(tokens.iter());
    let ast = parser.parse();

    let mut interpreter = Interpeter::new();
    interpreter.interpret(ast);
}
