use crate::error::ReadError;
use crate::r#trait::Reader;
use std::fmt::{Display, Formatter};

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    BeginObject,
    EndObject,
    BeginArray,
    EndArray,
    Colon,
    Comma,
    String(String),
    Number(f64),
    Null,
    Boolean(bool),
    Eof,
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::BeginObject => f.write_str("{"),
            Token::EndObject => f.write_str("}"),
            Token::BeginArray => f.write_str("["),
            Token::EndArray => f.write_str("]"),
            Token::Colon => f.write_str(":"),
            Token::Comma => f.write_str(","),
            Token::String(s) => f.write_fmt(format_args!("\"{}\"", s)),
            Token::Number(s) => f.write_fmt(format_args!("{}", s)),
            Token::Null => f.write_str("null"),
            Token::Boolean(s) => f.write_fmt(format_args!("{}", s)),
            Token::Eof => Ok(()),
        }
    }
}

pub fn fmt_print_tokens(tokens: &[Token]) {
    let mut indent = 0;
    let mut newline = true;
    for token in tokens {
        match token {
            Token::BeginArray => {
                println!("{token}");
                indent += 4;
                newline = true;
            }

            Token::BeginObject => {
                if newline {
                    print!("{}", " ".repeat(indent));
                }
                println!("{token}");
                indent += 4;
                newline = true;
            }

            Token::EndObject | Token::EndArray => {
                indent -= 4;
                println!();
                print!("{}{token}", " ".repeat(indent));
                newline = true;
            }
            Token::Comma => {
                println!(", ");
                newline = true;
            }
            Token::Colon => {
                print!(": ");
            }
            e => {
                if newline {
                    print!("{}", " ".repeat(indent));
                    newline = false;
                }
                print!("{e}");
            }
        }
    }
}

pub struct TokenReader<'a> {
    buffer: &'a [Token],
    pos: usize,
}

impl<'a> TokenReader<'a> {
    pub fn new(tokens: &'a [Token]) -> Self {
        Self {
            buffer: tokens,
            pos: 0,
        }
    }
}

pub struct TokenOwner {
    buffer: Vec<Option<Token>>,
    pos: usize,
}

impl TokenOwner {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            buffer: tokens.into_iter().map(Some).collect(),
            pos: 0,
        }
    }

    pub fn peek(&self) -> Result<&Token, ReadError> {
        match self.buffer.get(self.pos) {
            Some(Some(v)) => Ok(v),
            _ => Err(ReadError::Eof),
        }
    }

    pub fn next(&mut self) -> Result<Token, ReadError> {
        if self.pos >= self.buffer.len() {
            return Err(ReadError::Eof);
        }
        self.pos += 1;
        self.buffer[self.pos - 1].take().ok_or(ReadError::Eof)
    }
}

impl<'a> Reader for TokenReader<'a> {
    type Item = &'a Token;

    fn peek(&self) -> Result<Self::Item, ReadError> {
        self.buffer.get(self.pos).ok_or(ReadError::Eof)
    }

    fn next(&mut self) -> Result<Self::Item, ReadError> {
        self.pos += 1;
        self.buffer.get(self.pos - 1).ok_or(ReadError::Eof)
    }
}
