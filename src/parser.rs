use std::{iter::Peekable, slice::Iter};

use crate::ast::{
    BinaryOp, BinaryOpType, Assignment, Block, Call, Expr, Function, Global, IfStmt, Print, Program,
    Return, Statement, Time, UnaryOp, UnaryOpType, WhileLoop,
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
        let mut globals = Vec::new();
        while !self.has_ended() {
            let function = self.parse_function();
            globals.push(function);
        }
        Program { globals }
    }

    fn parse_function(&mut self) -> Global {
        self.advance();
        let name = self.parse_name();
        let parameters = self.parse_parameters();
        let block = self.parse_block();
        let function = Function {
            name,
            parameters,
            block,
        };
        Global::Function(function)
    }

    fn parse_parameters(&mut self) -> Vec<String> {
        self.advance_specific(&Token::LParen);
        let mut parameters = Vec::new();
        if self.peek() == &Token::RParen {
            self.advance();
            return parameters;
        }
        let first_parameter = self.parse_name();
        parameters.push(first_parameter);
        loop {
            match self.advance() {
                Token::RParen => return parameters,
                Token::Comma => (),
                other => panic!("Error while trying to parse parameter: {other:?}"),
            }
            let parameter = self.parse_name();
            parameters.push(parameter);
        }
    }

    fn parse_block(&mut self) -> Block {
        self.advance_specific(&Token::LCurly);
        let mut statements = Vec::new();
        while self.peek() != &Token::RCurly {
            let statement = self.parse_statement();
            statements.push(statement);
        }
        self.advance_specific(&Token::RCurly);
        Block { statements }
    }

    fn parse_statement(&mut self) -> Statement {
        match self.peek() {
            Token::If => self.parse_if(),
            Token::While => self.parse_while(),
            Token::Return => self.parse_return(),
            Token::Print => self.parse_print(),
            _ => {
                let expr = self.parse_expression();
                match self.advance() {
                    Token::Semicolon => {
                        Statement::Expr(expr)
                    }
                    Token::Equal => {
                        let name = match expr {
                            Expr::Name(name) => name,
                            _ => panic!("Cannot assign to {expr:?}")
                        };
                        let rhs = self.parse_expression();
                        self.advance_specific(&Token::Semicolon);
                        let assignment = Assignment { name, expr: rhs };
                        Statement::Assignment(assignment)
                    }
                    _ => panic!("Error while trying to parse statement"),
                }
            },
        }
    }

    fn parse_if(&mut self) -> Statement {
        self.advance_specific(&Token::If);
        let condition = self.parse_expression();
        let body = self.parse_block();
        let if_stmt = IfStmt { condition, body };
        Statement::If(if_stmt)
    }

    fn parse_while(&mut self) -> Statement {
        self.advance_specific(&Token::While);
        let condition = self.parse_expression();
        let body = self.parse_block();
        let while_loop = WhileLoop { condition, body };
        Statement::While(while_loop)
    }

    fn parse_return(&mut self) -> Statement {
        self.advance_specific(&Token::Return);
        let expr = self.parse_expression();
        self.advance_specific(&Token::Semicolon);
        let return_stmt = Return { expr };
        Statement::Return(return_stmt)
    }

    fn parse_print(&mut self) -> Statement {
        self.advance_specific(&Token::Print);
        self.advance_specific(&Token::LParen);
        let expr = self.parse_expression();
        self.advance_specific(&Token::RParen);
        self.advance_specific(&Token::Semicolon);
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
        let comparison_type = match self.peek() {
            Token::DoubleEqual => BinaryOpType::EqualTo,
            Token::NotEqual => BinaryOpType::NotEqualTo,
            Token::Less => BinaryOpType::LessThan,
            Token::LessEqual => BinaryOpType::LessThanOrEqualTo,
            Token::Greater => BinaryOpType::GreaterThan,
            Token::GreaterEqual => BinaryOpType::GreaterThanOrEqualTo,
            _ => return left,
        };
        self.advance();
        let right = self.parse_addition_and_subtraction();
        let op = BinaryOp {
            op_type: comparison_type,
            left: Box::new(left),
            right: Box::new(right),
        };
        Expr::BinaryOp(op)
    }

    fn parse_addition_and_subtraction(&mut self) -> Expr {
        let mut current = self.parse_multiplication_and_division();
        loop {
            let expr_type = match self.peek() {
                Token::Plus => BinaryOpType::Addition,
                Token::Minus => BinaryOpType::Subtraction,
                _ => return current,
            };
            self.advance();
            let right = self.parse_multiplication_and_division();
            let expr = BinaryOp {
                op_type: expr_type,
                left: Box::new(current),
                right: Box::new(right),
            };
            current = Expr::BinaryOp(expr);
        }
    }

    fn parse_multiplication_and_division(&mut self) -> Expr {
        let mut current = self.parse_unary();
        loop {
            let expr_type = match self.peek() {
                Token::Star => BinaryOpType::Multiplication,
                Token::Slash => BinaryOpType::Division,
                Token::Percent => BinaryOpType::Modulo,
                _ => return current,
            };
            self.advance();
            let right = self.parse_unary();
            let expr = BinaryOp {
                op_type: expr_type,
                left: Box::new(current),
                right: Box::new(right),
            };
            current = Expr::BinaryOp(expr);
        }
    }

    fn parse_unary(&mut self) -> Expr {
        match self.advance() {
            Token::LParen => {
                let expression = self.parse_expression();
                self.advance_specific(&Token::RParen);
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
            Token::Name(name) => {
                let n = name.to_string();
                self.parse_name_or_function(n)
            }
            Token::Time => {
                self.advance_specific(&Token::LParen);
                self.advance_specific(&Token::RParen);
                let time = Time {};
                Expr::Time(time)
            }
            Token::True => Expr::Literal(Value::Boolean(true)),
            Token::False => Expr::Literal(Value::Boolean(false)),
            other => panic!("Error while trying to parse expression: {other:?}"),
        }
    }

    fn parse_name_or_function(&mut self, name: String) -> Expr {
        match self.peek() {
            Token::LParen => {
                let arguments = self.parse_arguments();
                let call = Call { name, arguments };
                Expr::Call(call)
            }
            _ => Expr::Name(name),
        }
    }

    fn parse_arguments(&mut self) -> Vec<Expr> {
        self.advance_specific(&Token::LParen);
        let mut arguments = Vec::new();
        if self.peek() == &Token::RParen {
            self.advance();
            return arguments;
        }
        let first_argument = self.parse_expression();
        arguments.push(first_argument);

        loop {
            match self.advance() {
                Token::RParen => return arguments,
                Token::Comma => (),
                other => panic!("Error while trying to parse arguments: {other:?}"),
            }
            let argument = self.parse_expression();
            arguments.push(argument);
        }
    }

    fn has_ended(&mut self) -> bool {
        matches!(self.tokens.peek(), None)
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

    fn advance_specific(&mut self, expected: &Token) {
        match self.tokens.next() {
            Some(token) if token == expected => (),
            Some(other) => panic!("Expected {expected:?} but found {other:?}"),
            None => panic!("Unexpected end of file (expected {expected:?}"),
        }
    }
}
