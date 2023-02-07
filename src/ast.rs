#[derive(Debug)]
pub struct Program {
    pub globals: Vec<Global>,
}

#[derive(Debug)]
pub enum Global {
    Function(Function),
}

#[derive(Debug)]
pub struct Function {
    pub name: String,
    pub block: Block,
}

#[derive(Debug)]
pub enum Statement {
    Binding(Binding),
    Print(Print),
}

#[derive(Debug)]
pub struct Block {
    pub statements: Vec<Statement>,
}

#[derive(Debug)]
pub struct Binding {
    pub name: String,
    pub expression: Expression,
}

#[derive(Debug)]
pub struct Print {
    pub expression: Expression,
}

#[derive(Debug)]
pub enum Expression {
    Integer(i64),
    Name(String),

    Addition(Box<Expression>, Box<Expression>),
    Subtraction(Box<Expression>, Box<Expression>),
    Multiplication(Box<Expression>, Box<Expression>),
    Division(Box<Expression>, Box<Expression>),
    Negate(Box<Expression>),
}
