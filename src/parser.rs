use std::{iter::Peekable, slice::Iter};

use crate::ast::{
    BinaryOp, BinaryOpType, Binding, Block, Expr, Function, Global, IfStmt, Print, Program,
    Statement, UnaryOp, UnaryOpType, WhileLoop,
};
use crate::token::Token;
use crate::value::Value;

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
        Program { globals }
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
            Token::If => self.parse_if(),
            Token::While => self.parse_while(),
            Token::Let => self.parse_let(),
            Token::Print => self.parse_print(),
            other => panic!("Error while trying to parse statement: {other:?}"),
        }
    }

    fn parse_if(&mut self) -> Statement {
        self.advance_specific(Token::If);
        let condition = self.parse_expression();
        let body = self.parse_block();
        let if_stmt = IfStmt { condition, body };
        Statement::If(if_stmt)
    }

    fn parse_while(&mut self) -> Statement {
        self.advance_specific(Token::While);
        let condition = self.parse_expression();
        let body = self.parse_block();
        let while_loop = WhileLoop { condition, body };
        Statement::While(while_loop)
    }

    fn parse_let(&mut self) -> Statement {
        self.advance_specific(Token::Let);
        let name = self.parse_name();
        self.advance_specific(Token::Equal);
        let expr = self.parse_expression();
        self.advance_specific(Token::Semicolon);
        let binding = Binding { name, expr };
        Statement::Binding(binding)
    }

    fn parse_print(&mut self) -> Statement {
        self.advance_specific(Token::Print);
        self.advance_specific(Token::LParen);
        let expr = self.parse_expression();
        self.advance_specific(Token::RParen);
        self.advance_specific(Token::Semicolon);
        let print = Print { expr };
        Statement::Print(print)
    }

    fn parse_name(&mut self) -> String {
        match self.advance() {
            Token::Name(name) => name.clone(),
            other => panic!("Error while trying to parse name: {other:?}"),
        }
    }

    fn parse_expression(&mut self) -> Expr {
        self.parse_comparison()
    }

    fn parse_comparison(&mut self) -> Expr {
        let left = self.parse_addition_and_subtraction();
        let opt_comparison = match self.peek() {
            Token::DoubleEqual => Some(BinaryOpType::EqualTo),
            Token::NotEqual => Some(BinaryOpType::NotEqualTo),
            Token::Less => Some(BinaryOpType::LessThan),
            Token::LessEqual => Some(BinaryOpType::LessThanOrEqualTo),
            Token::Greater => Some(BinaryOpType::GreaterThan),
            Token::GreaterEqual => Some(BinaryOpType::GreaterThanOrEqualTo),
            _ => None,
        };
        match opt_comparison {
            Some(comparison) => {
                self.advance();
                let right = self.parse_addition_and_subtraction();
                let op = BinaryOp {
                    op_type: comparison,
                    left: Box::new(left),
                    right: Box::new(right),
                };
                Expr::BinaryOp(op)
            }
            None => left,
        }
    }

    fn parse_addition_and_subtraction(&mut self) -> Expr {
        let mut current = self.parse_multiplication_and_division();
        loop {
            match self.peek() {
                Token::Plus => {
                    self.advance();
                    let right = self.parse_multiplication_and_division();
                    let addition = BinaryOp {
                        op_type: BinaryOpType::Addition,
                        left: Box::new(current),
                        right: Box::new(right),
                    };
                    current = Expr::BinaryOp(addition);
                }
                Token::Minus => {
                    self.advance();
                    let right = self.parse_multiplication_and_division();
                    let subtraction = BinaryOp {
                        op_type: BinaryOpType::Subtraction,
                        left: Box::new(current),
                        right: Box::new(right),
                    };
                    current = Expr::BinaryOp(subtraction);
                }
                _ => return current,
            }
        }
    }

    fn parse_multiplication_and_division(&mut self) -> Expr {
        let mut current = self.parse_unary();
        loop {
            match self.peek() {
                Token::Star => {
                    self.advance();
                    let right = self.parse_unary();
                    let multiplication = BinaryOp {
                        op_type: BinaryOpType::Multiplication,
                        left: Box::new(current),
                        right: Box::new(right),
                    };
                    current = Expr::BinaryOp(multiplication);
                }
                Token::Slash => {
                    self.advance();
                    let right = self.parse_unary();
                    let division = BinaryOp {
                        op_type: BinaryOpType::Division,
                        left: Box::new(current),
                        right: Box::new(right),
                    };
                    current = Expr::BinaryOp(division);
                }
                _ => return current,
            }
        }
    }

    fn parse_unary(&mut self) -> Expr {
        match self.advance() {
            Token::LParen => {
                let expression = self.parse_expression();
                self.advance_specific(Token::RParen);
                expression
            }
            Token::Minus => {
                let unary = self.parse_unary();
                let negate = UnaryOp {
                    op_type: UnaryOpType::Negate,
                    expr: Box::new(unary),
                };
                Expr::UnaryOp(negate)
            }
            Token::Integer(integer) => Expr::Literal(Value::Integer(*integer)),
            Token::Name(name) => Expr::Name(name.to_string()),
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
