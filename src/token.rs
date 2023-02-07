#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    End,

    Function,
    Let,
    Print,

    LParen,
    RParen,
    LCurly,
    RCurly,

    Equals,
    Comma,
    Plus,
    Minus,
    Star,
    Slash,
    Semicolon,

    Name(String),
    Integer(i64),
}
