#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    End,

    // Keywords
    Function,
    If,
    While,
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
    Percent,
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
