use crate::{lexer::Lexer, parser::Parser, interpreter::Interpeter, value::Value};

pub struct Salt {
}

impl Salt {
    pub fn new() -> Self {
        Self {
        }
    }

    pub fn run(&self, source: &str)  -> Value {
        let mut lexer = Lexer::new(source.chars());
        let tokens = lexer.lex();
        let mut parser = Parser::new(tokens.iter());
        let ast = parser.parse();
        let mut interpreter = Interpeter::new();
        interpreter.load(ast);
        interpreter.call_function("main")
    }
}
