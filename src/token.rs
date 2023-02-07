#[derive(Debug)]
pub enum Token {
    End,

    Function,
    Let,

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
