mod ast;
mod environment;
mod interpreter;
mod lexer;
mod parser;
mod salt;
mod tests;
mod token;
mod value;

use salt::Salt;

use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_name = args.get(1).expect("usage: salt <filename>");
    let source = fs::read_to_string(file_name).expect("Unable to read file!");

    let salt = Salt::new();
    salt.run(&source);
}
