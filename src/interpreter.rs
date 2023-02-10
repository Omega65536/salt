use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::ast::{
    BinaryOp, BinaryOpType, Binding, Block, Expr, Function, Global, IfStmt, Print, Program, Return,
    Statement, Time, UnaryOp, UnaryOpType, WhileLoop,
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

    pub fn call_function(&mut self, function_name: &str) -> Value {
        let main_function = self
            .functions
            .get(function_name)
            .expect("Could not find function");
        let mut env = Environment::new();
        let main_arguments = Vec::new();
        self.interpret_function(main_function, main_arguments, &mut env)
    }

    pub fn load(&mut self, program: Program) {
        for global in program.globals {
            match global {
                Global::Function(function) => {
                    self.functions.insert(function.name.clone(), function)
                }
            };
        }
    }

    fn interpret_function(
        &self,
        function: &Function,
        arguments: Vec<Value>,
        env: &mut Environment,
    ) -> Value {
        //println!("<Interpreting {:?} with {:?} = {:?}>", function.name, function.parameters, arguments);
        if function.parameters.len() != arguments.len() {
            panic!("Invalid arity!");
        }
        for (parameter, argument) in function.parameters.iter().zip(arguments) {
            env.set(parameter.to_string(), argument);
        }
        self.interpret_block(&function.block, env)
            .unwrap_or(Value::Unit)
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
            Value::Boolean(true) => (),
            Value::Boolean(false) => return None,
            _ => panic!("Type error"),
        }
        self.interpret_block(&if_stmt.body, env)
    }

    fn interpret_while(&self, while_loop: &WhileLoop, env: &mut Environment) -> Option<Value> {
        loop {
            let condition = self.interpret_expression(&while_loop.condition, env);
            match condition {
                Value::Boolean(true) => (),
                Value::Boolean(false) => return None,
                _ => panic!("Type error"),
            }
            if let Some(value) = self.interpret_block(&while_loop.body, env) {
                return Some(value);
            }
        }
    }

    fn interpret_block(&self, block: &Block, env: &mut Environment) -> Option<Value> {
        for statement in &block.statements {
            if let Some(value) = self.interpret_statement(statement, env) {
                return Some(value);
            }
        }
        None
    }

    fn interpret_return(&self, return_stmt: &Return, env: &Environment) -> Option<Value> {
        let evaluated = self.interpret_expression(&return_stmt.expr, env);
        Some(evaluated)
    }

    fn interpret_binding(&self, binding: &Binding, env: &mut Environment) -> Option<Value> {
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
                let arguments_evaluated = call
                    .arguments
                    .iter()
                    .map(|expr| self.interpret_expression(expr, env))
                    .collect();
                self.interpret_function(function, arguments_evaluated, &mut new_env)
            }
            Expr::UnaryOp(unary_op) => self.interpret_unary_op(unary_op, env),
            Expr::BinaryOp(binary_op) => self.interpret_binary_op(binary_op, env),
            Expr::Name(name) => {
                let evaluated = env.get(name).expect("No such variable");
                *evaluated
            }
            Expr::Time(time) => self.interpret_time(time, env),
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

    fn interpret_time(&self, _time: &Time, _env: &Environment) -> Value {
        let millis = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();
        Value::Integer(millis as i64)
    }
}
