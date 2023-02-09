mod ast;
mod interpreter;
mod lexer;
mod parser;
mod token;
mod value;
mod environment;

use crate::interpreter::Interpeter;
use crate::lexer::Lexer;
use crate::parser::Parser;
use std::{fs, env};

fn main() {
    let args: Vec<String> = env::args().collect();
    let source_file = args.get(1).expect("usage: salt <filename>");

    let program = fs::read_to_string(source_file).expect("Unable to read file!");

    let mut lexer = Lexer::new(program.chars());
    let tokens = lexer.lex();

    let mut parser = Parser::new(tokens.iter());
    let ast = parser.parse();

    let mut interpreter = Interpeter::new();
    interpreter.interpret(ast);
}
