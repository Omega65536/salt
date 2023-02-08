use std::collections::HashMap;

use crate::ast::{
    BinaryOpType, Binding, Expr, Function, Global, IfStmt, Print, Program, Statement, UnaryOpType,
    WhileLoop,
};
use crate::value::Value;

pub struct Interpeter {
    environment: HashMap<String, Value>,
}

impl Interpeter {
    pub fn new() -> Self {
        Self {
            environment: HashMap::new(),
        }
    }

    pub fn interpret(&mut self, program: Program) {
        let main_function = &program.globals[0];
        match main_function {
            Global::Function(function) => self.interpret_function(function),
        }
    }

    fn interpret_function(&mut self, function: &Function) {
        println!("<Interpreting {:?}>", function.name);
        for statement in &function.block.statements {
            self.interpret_statement(statement);
        }
    }

    fn interpret_statement(&mut self, statement: &Statement) {
        match statement {
            Statement::If(if_stmt) => self.interpret_if(if_stmt),
            Statement::While(while_loop) => self.interpret_while(while_loop),
            Statement::Binding(binding) => self.interpret_binding(binding),
            Statement::Print(print) => self.interpret_print(print),
        }
    }

    fn interpret_if(&mut self, if_stmt: &IfStmt) {
        let condition = self.interpret_expression(&if_stmt.condition);
        match condition {
            Value::Boolean(true) => {
                for statement in &if_stmt.body.statements {
                    self.interpret_statement(statement);
                }
            }
            Value::Boolean(false) => {}
            _ => panic!("Type error"),
        }
    }

    fn interpret_while(&mut self, while_loop: &WhileLoop) {
        loop {
            let condition = self.interpret_expression(&while_loop.condition);
            match condition {
                Value::Boolean(true) => {
                    for statement in &while_loop.body.statements {
                        self.interpret_statement(statement);
                    }
                }
                Value::Boolean(false) => break,
                _ => panic!("Type error"),
            }
        }
    }

    fn interpret_binding(&mut self, binding: &Binding) {
        let evaluated = self.interpret_expression(&binding.expr);
        self.environment.insert(binding.name.clone(), evaluated);
    }

    fn interpret_print(&self, print: &Print) {
        let evaluated = self.interpret_expression(&print.expr);
        match evaluated {
            Value::Integer(v) => println!("{v}"),
            Value::Boolean(v) => println!("{v}"),
        }
    }

    fn interpret_expression(&self, expression: &Expr) -> Value {
        match expression {
            Expr::Literal(v) => *v,
            Expr::UnaryOp(unary_op_type, expr) => self.interpret_unary_op(unary_op_type, expr),
            Expr::BinaryOp(binary_op_type, left, right) => {
                self.interpret_binary_op(binary_op_type, left, right)
            }
            Expr::Name(name) => {
                let evaluated = self.environment.get(name).unwrap();
                *evaluated
            }
        }
    }

    fn interpret_unary_op(&self, unary_op_type: &UnaryOpType, expr: &Expr) -> Value {
        let expr_evaluated = self.interpret_expression(expr);
        match unary_op_type {
            UnaryOpType::Negate => match expr_evaluated {
                Value::Integer(v) => Value::Integer(-v),
                _ => panic!("Type error"),
            },
        }
    }

    fn interpret_binary_op(
        &self,
        binary_op_type: &BinaryOpType,
        left: &Expr,
        right: &Expr,
    ) -> Value {
        let left_evaluated = self.interpret_expression(left);
        let right_evaluated = self.interpret_expression(right);
        match binary_op_type {
            BinaryOpType::Addition => match (left_evaluated, right_evaluated) {
                (Value::Integer(l), Value::Integer(r)) => Value::Integer(l + r),
                _ => panic!("Type error"),
            },
            BinaryOpType::Subtraction => match (left_evaluated, right_evaluated) {
                (Value::Integer(l), Value::Integer(r)) => Value::Integer(l - r),
                _ => panic!("Type error"),
            },
            BinaryOpType::Multiplication => match (left_evaluated, right_evaluated) {
                (Value::Integer(l), Value::Integer(r)) => Value::Integer(l * r),
                _ => panic!("Type error"),
            },
            BinaryOpType::Division => match (left_evaluated, right_evaluated) {
                (Value::Integer(l), Value::Integer(r)) => Value::Integer(l / r),
                _ => panic!("Type error"),
            },
            BinaryOpType::EqualTo => match (left_evaluated, right_evaluated) {
                (Value::Integer(l), Value::Integer(r)) => Value::Boolean(l == r),
                _ => panic!("Type error"),
            },
            BinaryOpType::NotEqualTo => match (left_evaluated, right_evaluated) {
                (Value::Integer(l), Value::Integer(r)) => Value::Boolean(l != r),
                _ => panic!("Type error"),
            },
            BinaryOpType::LessThan => match (left_evaluated, right_evaluated) {
                (Value::Integer(l), Value::Integer(r)) => Value::Boolean(l < r),
                _ => panic!("Type error"),
            },
            BinaryOpType::LessThanOrEqualTo => match (left_evaluated, right_evaluated) {
                (Value::Integer(l), Value::Integer(r)) => Value::Boolean(l <= r),
                _ => panic!("Type error"),
            },
            BinaryOpType::GreaterThan => match (left_evaluated, right_evaluated) {
                (Value::Integer(l), Value::Integer(r)) => Value::Boolean(l > r),
                _ => panic!("Type error"),
            },
            BinaryOpType::GreaterThanOrEqualTo => match (left_evaluated, right_evaluated) {
                (Value::Integer(l), Value::Integer(r)) => Value::Boolean(l >= r),
                _ => panic!("Type error"),
            },
        }
    }
}
