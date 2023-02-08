use crate::value::Value;

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
    If(IfStmt),
    While(WhileLoop),
    Binding(Binding),
    Print(Print),
}

#[derive(Debug)]
pub struct IfStmt {
    pub condition: Expr,
    pub body: Block,
}

#[derive(Debug)]
pub struct WhileLoop {
    pub condition: Expr,
    pub body: Block,
}

#[derive(Debug)]
pub struct Block {
    pub statements: Vec<Statement>,
}

#[derive(Debug)]
pub struct Binding {
    pub name: String,
    pub expr: Expr,
}

#[derive(Debug)]
pub struct Print {
    pub expr: Expr,
}
#[derive(Debug)]
pub enum Expr {
    Literal(Value),
    Name(String),

    UnaryOp(UnaryOp),
    BinaryOp(BinaryOp),
}

#[derive(Debug)]
pub struct UnaryOp {
    pub op_type: UnaryOpType,
    pub expr: Box<Expr>,
}

#[derive(Debug)]
pub struct BinaryOp {
    pub op_type: BinaryOpType,
    pub left: Box<Expr>,
    pub right: Box<Expr>,
}

#[derive(Debug)]
pub enum UnaryOpType {
    Negate,
}

#[derive(Debug)]
pub enum BinaryOpType {
    Addition,
    Subtraction,
    Multiplication,
    Division,

    EqualTo,
    NotEqualTo,
    LessThan,
    LessThanOrEqualTo,
    GreaterThan,
    GreaterThanOrEqualTo,
}
