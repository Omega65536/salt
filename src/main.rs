mod token;
mod lexer;
mod ast;
mod parser;
mod value;
mod interpreter;

use std::fs;
use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::interpreter::Interpeter;

fn main() {
    let program = fs::read_to_string("example.salt").expect("Unable to read file!");

    let mut lexer = Lexer::new(program.chars());
    let tokens = lexer.lex();

    let mut parser = Parser::new(tokens.iter());
    let ast = parser.parse();

    let mut interpreter = Interpeter::new();
    interpreter.interpret(ast);
}
