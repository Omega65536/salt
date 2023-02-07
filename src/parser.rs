use std::{iter::Peekable, slice::Iter};

use crate::ast::{Binding, Block, Expression, Function, Global, Statement, Print, Program};
use crate::token::Token;

pub struct Parser<'a> {
    tokens: Peekable<Iter<'a, Token>>,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: Iter<'a, Token>) -> Self {
        Self {
            tokens: tokens.peekable(),
        }
    }

    pub fn parse(&mut self) -> Program {
        let function = Global::Function(self.parse_function());
        let globals = vec![function];
        let program = Program {
            globals
        };
        program
    }

    fn parse_function(&mut self) -> Function {
        self.advance();
        let name = self.parse_name();
        self.advance_specific(Token::LParen);
        self.advance_specific(Token::RParen);
        let block = self.parse_block();
        Function { name, block }
    }

    fn parse_block(&mut self) -> Block {
        self.advance_specific(Token::LCurly);
        let mut statements = Vec::new();
        while self.peek() != &Token::RCurly {
            let statement = self.parse_statement();
            statements.push(statement);
        }
        self.advance_specific(Token::RCurly);
        Block { statements }
    }

    fn parse_statement(&mut self) -> Statement {
        match self.peek() {
            Token::Let => self.parse_let(),
            Token::Print => self.parse_print(),
            other => panic!("Error while trying to parse statement: {other:?}"),
        }
    }

    fn parse_let(&mut self) -> Statement {
        self.advance_specific(Token::Let);
        let name = self.parse_name();
        self.advance_specific(Token::Equals);
        let expression = self.parse_expression();
        self.advance_specific(Token::Semicolon);
        let binding = Binding {
            name,
            expression,
        };
        Statement::Binding(binding)
    }

    fn parse_print(&mut self) -> Statement {
        self.advance_specific(Token::Print);
        self.advance_specific(Token::LParen);
        let expression = self.parse_expression();
        self.advance_specific(Token::RParen);
        self.advance_specific(Token::Semicolon);
        let print = Print {
            expression,
        };
        Statement::Print(print)
    }

    fn parse_name(&mut self) -> String {
        match self.advance() {
            Token::Name(name) => name.clone(),
            other => panic!("Error while trying to parse name: {other:?}"),
        }
    }

    fn parse_expression(&mut self) -> Expression {
        self.parse_addition_and_subtraction()
    }

    fn parse_addition_and_subtraction(&mut self) -> Expression {
        let mut current = self.parse_multiplication_and_division();
        loop {
            match self.peek() {
                Token::Plus => {
                    self.advance();
                    let right = self.parse_multiplication_and_division();
                    current = Expression::Addition(Box::new(current), Box::new(right));
                }
                Token::Minus => {
                    self.advance();
                    let right = self.parse_multiplication_and_division();
                    current = Expression::Subtraction(Box::new(current), Box::new(right));
                }
                _ => return current,
            }
        }
    }

    fn parse_multiplication_and_division(&mut self) -> Expression {
        let mut current = self.parse_unary();
        loop {
            match self.peek() {
                Token::Star => {
                    self.advance();
                    let right = self.parse_unary();
                    current = Expression::Multiplication(Box::new(current), Box::new(right));
                }
                Token::Slash => {
                    self.advance();
                    let right = self.parse_unary();
                    current = Expression::Division(Box::new(current), Box::new(right));
                }
                _ => return current,
            }
        }
    }

    fn parse_unary(&mut self) -> Expression {
        match self.advance() {
            Token::LParen => {
                let expression = self.parse_expression();
                self.advance_specific(Token::RParen);
                expression
            }
            Token::Minus => {
                let unary = self.parse_unary();
                Expression::Negate(Box::new(unary))
            }
            Token::Integer(integer) => Expression::Integer(*integer),
            Token::Name(name) => Expression::Name(name.to_string()),
            other => panic!("Error while trying to parse expression: {other:?}"),
        }
    }

    fn peek(&mut self) -> &Token {
        match self.tokens.peek() {
            Some(token) => token,
            None => panic!("Unexpected end of file"),
        }
    }

    fn advance(&mut self) -> &Token {
        match self.tokens.next() {
            Some(token) => token,
            None => panic!("Unexpected end of file"),
        }
    }

    fn advance_specific(&mut self, expected: Token) {
        match self.tokens.next() {
            Some(token) if token == &expected => (),
            Some(other) => panic!("Expected {expected:?} but found {other:?}"),
            None => panic!("Unexpected end of file (expected {expected:?}"),
        }
    }
}
