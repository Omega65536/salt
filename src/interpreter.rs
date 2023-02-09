use std::collections::HashMap;

use crate::ast::{
    BinaryOp, BinaryOpType, Binding, Expr, Function, Global, IfStmt, Print, Program, Statement,
    UnaryOp, UnaryOpType, WhileLoop, Return,
};
use crate::environment::Environment;
use crate::value::Value;

pub struct Interpeter {
    functions: HashMap<String, Function>,
}

impl Interpeter {
    pub fn new() -> Self {
        Self {
            functions: HashMap::new(),
        }
    }

    pub fn interpret(&mut self, program: Program) {
        for global in program.globals {
            match global {
                Global::Function(function) => {
                    self.functions.insert(function.name.clone(), function)
                }
            };
        }
        let main_function = self
            .functions
            .get("main")
            .expect("Could not find main function");
        let mut env = Environment::new();
        let main_arguments = Vec::new();
        self.interpret_function(main_function, main_arguments, &mut env);
    }

    fn interpret_function(&self, function: &Function, arguments: Vec<Value>, env: &mut Environment)  -> Value {
        //println!("<Interpreting {:?} with {:?} = {:?}>", function.name, function.parameters, arguments);
        if function.parameters.len() != arguments.len() {
            panic!("Invalid arity!");
        }
        for (parameter, argument) in function.parameters.iter().zip(arguments) {
            env.set(parameter.to_string(), argument);
        }
        for statement in &function.block.statements {
            if let Some(value) = self.interpret_statement(statement, env) {
                return value
            }
        }
        Value::Unit
    }

    fn interpret_statement(&self, statement: &Statement, env: &mut Environment) -> Option<Value> {
        match statement {
            Statement::If(if_stmt) => self.interpret_if(if_stmt, env),
            Statement::While(while_loop) => self.interpret_while(while_loop, env),
            Statement::Return(return_stmt) => self.interpret_return(return_stmt, env),
            Statement::Binding(binding) => self.interpret_binding(binding, env),
            Statement::Print(print) => self.interpret_print(print, env),
        }
    }

    fn interpret_if(&self, if_stmt: &IfStmt, env: &mut Environment) -> Option<Value> {
        let condition = self.interpret_expression(&if_stmt.condition, env);
        match condition {
            Value::Boolean(true) => {
                for statement in &if_stmt.body.statements {
                    if let Some(value) = self.interpret_statement(statement, env) {
                        return Some(value)
                    }
                }
            }
            Value::Boolean(false) => {}
            _ => panic!("Type error"),
        }
        None
    }

    fn interpret_while(&self, while_loop: &WhileLoop, env: &mut Environment) -> Option<Value> {
        loop {
            let condition = self.interpret_expression(&while_loop.condition, env);
            match condition {
                Value::Boolean(true) => {
                    for statement in &while_loop.body.statements {
                        if let Some(value) = self.interpret_statement(statement, env) {
                            return Some(value)
                        }
                    }
                }
                Value::Boolean(false) => break,
                _ => panic!("Type error"),
            }
        }
        None
    }

    fn interpret_return(&self, return_stmt: &Return, env: &Environment) -> Option<Value> {
        let evaluated = self.interpret_expression(&return_stmt.expr, env);
        Some(evaluated)
    }

    fn interpret_binding(&self, binding: &Binding, env: &mut Environment) -> Option<Value>{
        let evaluated = self.interpret_expression(&binding.expr, env);
        env.set(binding.name.clone(), evaluated);
        None
    }

    fn interpret_print(&self, print: &Print, env: &mut Environment) -> Option<Value> {
        let evaluated = self.interpret_expression(&print.expr, env);
        match evaluated {
            Value::Unit => println!("()"),
            Value::Boolean(v) => println!("{v}"),
            Value::Integer(v) => println!("{v}"),
        }
        None
    }

    fn interpret_expression(&self, expression: &Expr, env: &Environment) -> Value {
        match expression {
            Expr::Literal(v) => *v,
            Expr::Call(call) => {
                let function = self.functions.get(&call.name).expect("No such function");
                let mut new_env = Environment::new();
                let arguments_evaluated = call.arguments.iter().map(
                    |expr| self.interpret_expression(expr, env)
                ).collect();
                self.interpret_function(function, arguments_evaluated, &mut new_env)
            }
            Expr::UnaryOp(unary_op) => self.interpret_unary_op(unary_op, env),
            Expr::BinaryOp(binary_op) => self.interpret_binary_op(binary_op, env),
            Expr::Name(name) => {
                let evaluated = env.get(name).expect("No such variable");
                *evaluated
            }
        }
    }

    fn interpret_unary_op(&self, op: &UnaryOp, env: &Environment) -> Value {
        let expr_evaluated = self.interpret_expression(&op.expr, env);
        match op.op_type {
            UnaryOpType::Negate => match expr_evaluated {
                Value::Integer(v) => Value::Integer(-v),
                _ => panic!("Type error"),
            },
        }
    }

    fn interpret_binary_op(&self, op: &BinaryOp, env: &Environment) -> Value {
        let left_evaluated = self.interpret_expression(&op.left, env);
        let right_evaluated = self.interpret_expression(&op.right, env);
        match op.op_type {
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
            BinaryOpType::Modulo => match (left_evaluated, right_evaluated) {
                (Value::Integer(l), Value::Integer(r)) => Value::Integer(l % r),
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
