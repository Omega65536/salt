use crate::{lexer::Lexer, parser::Parser, interpreter::Interpeter};

pub struct Salt {
}

impl Salt {
    pub fn new() -> Self {
        Self {
        }
    }

    pub fn run(&self, source: String) {
        let mut lexer = Lexer::new(source.chars());
        let tokens = lexer.lex();
        let mut parser = Parser::new(tokens.iter());
        let ast = parser.parse();
        let mut interpreter = Interpeter::new();
        interpreter.interpret(ast);
    }
}
