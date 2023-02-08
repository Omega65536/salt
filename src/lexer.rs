use std::iter::Peekable;
use std::str::Chars;

use crate::token::Token;

pub struct Lexer<'a> {
    chars: Peekable<Chars<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(chars: Chars<'a>) -> Self {
        Self {
            chars: chars.peekable(),
        }
    }

    pub fn lex(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        loop {
            match self.next_token() {
                Some(Token::End) => break,
                token => tokens.extend(token),
            }
        }
        tokens
    }

    fn next_token(&mut self) -> Option<Token> {
        match self.chars.peek() {
            Some(ch) => match ch {
                ' ' | '\n' | '\t' => {
                    self.chars.next();
                    None
                }
                '#' => {
                    loop {
                        match self.chars.next() {
                            Some('\n') | None => break,
                            _ => (),
                        }
                    }
                    None
                }
                ch if ch.is_ascii_alphabetic() => Some(self.next_name_or_keyword()),
                ch if ch.is_ascii_digit() => Some(self.next_integer()),
                _ => Some(self.next_symbol()),
            },
            None => Some(Token::End),
        }
    }

    fn next_symbol(&mut self) -> Token {
        match self.chars.next() {
            Some(ch) => match ch {
                '+' => Token::Plus,
                '-' => Token::Minus,
                '*' => Token::Star,
                '/' => Token::Slash,
                '(' => Token::LParen,
                ')' => Token::RParen,
                '{' => Token::LCurly,
                '}' => Token::RCurly,
                ';' => Token::Semicolon,
                '=' => match self.chars.peek() {
                    Some('=') => {
                        self.chars.next();
                        Token::DoubleEqual
                    }
                    _ => Token::Equal,
                },
                '!' => match self.chars.peek() {
                    Some('=') => {
                        self.chars.next();
                        Token::NotEqual
                    }
                    _ => Token::Bang,
                },
                '<' => match self.chars.peek() {
                    Some('=') => {
                        self.chars.next();
                        Token::LessEqual
                    }
                    _ => Token::Less,
                },
                '>' => match self.chars.peek() {
                    Some('=') => {
                        self.chars.next();
                        Token::GreaterEqual
                    }
                    _ => Token::Greater,
                },
                _ => panic!("Unknown symbol: {ch:?}"),
            },
            None => panic!("Internal Error"),
        }
    }

    fn next_name_or_keyword(&mut self) -> Token {
        let mut name = String::new();
        while let Some(ch) = self.chars.peek() {
            if !ch.is_ascii_alphabetic() {
                break;
            }
            name.push(*ch);
            self.chars.next();
        }
        match name.as_str() {
            "fn" => Token::Function,
            "if" => Token::If,
            "while" => Token::While,
            "let" => Token::Let,
            "print" => Token::Print,
            _ => Token::Name(name),
        }
    }

    fn next_integer(&mut self) -> Token {
        let mut buffer = String::new();
        while let Some(ch) = self.chars.peek() {
            if !ch.is_ascii_digit() {
                break;
            }
            buffer.push(*ch);
            self.chars.next();
        }
        let i = buffer.parse().unwrap();
        Token::Integer(i)
    }
}
