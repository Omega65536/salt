#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    End,

    // Keywords
    Function,
    If,
    Let,
    Print,

    // Delimiter
    LParen,
    RParen,
    LCurly,
    RCurly,
    Semicolon,

    // Opetator
    Equal,
    Plus,
    Minus,
    Star,
    Slash,
    Bang,

    DoubleEqual,
    NotEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,

    // Name + Literals
    Name(String),
    Integer(i64),
}
