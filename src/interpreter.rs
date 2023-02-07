use std::collections::HashMap;

use crate::{ast::{Program, Function, Global, Statement, Print, Expression, Binding}, value::Value};

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
            Global::Function(function) => self.interpret_function(&function), 
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
            Statement::Binding(binding) => self.interpret_binding(binding),
            Statement::Print(print) => self.interpret_print(print),
        }
    }

    fn interpret_binding(&mut self, binding: &Binding) {
        let evaluated = self.interpret_expression(&binding.expression);
        self.environment.insert(binding.name.clone(), evaluated);
    }

    fn interpret_print(&self, print: &Print) {
        let evaluated = self.interpret_expression(&print.expression);
        match evaluated {
            Value::Integer(i) => println!("{}", i),
        }
    }

    fn interpret_expression(&self, expression: &Expression) -> Value {
        match expression {
            Expression::Integer(i) => Value::Integer(*i),
            Expression::Negate(expression) => {
                let evaluated = self.interpret_expression(expression);
                match evaluated {
                    Value::Integer(i) => Value::Integer(-i),
                }
            },
            Expression::Addition(left, right) => {
                let left_evaluated = self.interpret_expression(left);
                let right_evaluated = self.interpret_expression(right);
                match (left_evaluated, right_evaluated) {
                    (Value::Integer(a), Value::Integer(b)) => Value::Integer(a + b),
                    _ => panic!("Type error"),
                }
            }
            Expression::Subtraction(left, right) => {
                let left_evaluated = self.interpret_expression(left);
                let right_evaluated = self.interpret_expression(right);
                match (left_evaluated, right_evaluated) {
                    (Value::Integer(a), Value::Integer(b)) => Value::Integer(a - b),
                    _ => panic!("Type error"),
                }
            }
            Expression::Multiplication(left, right) => {
                let left_evaluated = self.interpret_expression(left);
                let right_evaluated = self.interpret_expression(right);
                match (left_evaluated, right_evaluated) {
                    (Value::Integer(a), Value::Integer(b)) => Value::Integer(a * b),
                    _ => panic!("Type error"),
                }
            }
            Expression::Division(left, right) => {
                let left_evaluated = self.interpret_expression(left);
                let right_evaluated = self.interpret_expression(right);
                match (left_evaluated, right_evaluated) {
                    (Value::Integer(a), Value::Integer(b)) => Value::Integer(a / b),
                    _ => panic!("Type error"),
                }
            }
            Expression::Name(name) => {
                let evaluated = self.environment.get(name).unwrap();
                *evaluated
            }
        }
    }
}
