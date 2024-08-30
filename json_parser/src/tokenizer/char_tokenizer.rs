use crate::error::ReadError;
use crate::r#trait::{JsonHandler, Reader, StreamToken};
use crate::reader::CharReader;
use crate::token::Token;
use std::str::Chars;

pub struct CharTokenizer<'a> {
    reader: CharReader<'a>,
}

impl<'a> StreamToken for CharTokenizer<'a> {
    fn read_token(&mut self) -> Result<Token, ReadError> {
        let mut char = self.reader.peek();
        loop {
            match char {
                Ok(v) if v.is_ascii_whitespace() => {
                    let _ = self.reader.next();
                    char = self.reader.peek();
                    continue;
                }
                _ => break,
            }
        }
        match char {
            Ok(v) => match v {
                '{' => {
                    let _ = self.reader.next();
                    Ok(Token::BeginObject)
                }
                '}' => {
                    let _ = self.reader.next();
                    Ok(Token::EndObject)
                }
                '[' => {
                    let _ = self.reader.next();
                    Ok(Token::BeginArray)
                }
                ']' => {
                    let _ = self.reader.next();
                    Ok(Token::EndArray)
                }
                ':' => {
                    let _ = self.reader.next();
                    Ok(Token::Colon)
                }
                ',' => {
                    let _ = self.reader.next();
                    Ok(Token::Comma)
                }
                '"' => self.read_string(),
                'n' => self.read_null(),
                't' | 'f' => self.read_boolean(),
                c if c.is_ascii_digit() || c == '-' => self.read_number(),
                c => Err(ReadError::IllegalChar(c)),
            },
            Err(ReadError::Eof) => Ok(Token::Eof),
            Err(e) => Err(e),
        }
    }
}
impl<'a> JsonHandler<&'a str> for CharTokenizer<'a> {
    fn new(buffer: &'a str) -> Self {
        Self {
            reader: CharReader::new(buffer.chars()),
        }
    }
}

impl<'a> CharTokenizer<'a> {
    fn read_string(&mut self) -> Result<Token, ReadError> {
        let mut buffer = String::with_capacity(16);
        let mut escape = false;

        let _ = self.reader.next(); // Skip the opening quote

        loop {
            match self.reader.next() {
                Ok('"') if !escape => break,
                Ok(c) if escape => {
                    buffer.push(match c {
                        '/' => '/',
                        '\\' => '\\',
                        't' => '\t',
                        'n' => '\n',
                        'r' => '\r',
                        _ => return Err(ReadError::IllegalEscape),
                    });
                    escape = false;
                }
                Ok('\\') if !escape => escape = true,
                Ok(c) => {
                    buffer.push(c);
                    escape = false;
                }
                Err(e) => return Err(e),
            }
        }
        Ok(Token::String(buffer))
    }

    fn read_string1(&mut self) -> Result<Token, ReadError> {
        let mut buffer = String::with_capacity(16);
        let mut escape = false;

        let _ = self.reader.next();
        let mut c = self.reader.next();

        loop {
            match c {
                Ok(v) => match v {
                    '"' => {
                        if escape {
                            buffer.push(v);
                            escape = false;
                        } else {
                            break;
                        }
                    }
                    mut c => {
                        if escape {
                            match c {
                                '/' => c = '/',
                                '\\' => c = '\\',
                                't' => c = '\t',
                                'n' => c = '\n',
                                'r' => c = '\r',
                                _ => return Err(ReadError::IllegalEscape),
                            }
                            escape = false;
                            buffer.push(c);
                        } else if c == '\\' {
                            escape = true;
                        } else {
                            buffer.push(c);
                        }
                    }
                },
                Err(e) => return Err(e),
            }
            c = self.reader.next();
        }
        Ok(Token::String(buffer))
    }

    fn read_number(&mut self) -> Result<Token, ReadError> {
        enum State {
            Integer,
            Fraction,
            Exponent,
        }

        let mut value = 0.;
        let mut sign = 1.;
        let mut rate = 0.1;

        let mut state = State::Integer;

        let mut exponent_sign = 1_i32;
        let mut exponent_value = 0;
        let mut c = self.reader.peek();

        if let Ok(v) = c {
            if v == '-' {
                sign = -1.;
                let _ = self.reader.next();
                c = self.reader.peek();
            }
        }

        loop {
            match c {
                Err(_) => break,
                Ok(v @ '0'..='9') => match state {
                    State::Integer => value = value * 10. + (v as u32 - '0' as u32) as f64,
                    State::Fraction => {
                        value += rate * (v as u32 - '0' as u32) as f64;
                        rate *= 0.1;
                    }
                    State::Exponent => {
                        exponent_value = exponent_value * 10 + (v as i32 - '0' as i32)
                    }
                },
                Ok('.') => {
                    if let State::Integer = state {
                        state = State::Fraction;
                    } else {
                        return Err(ReadError::IllegalToken);
                    }
                }
                Ok('e' | 'E') => {
                    if let State::Exponent = state {
                        return Err(ReadError::IllegalToken);
                    }
                    state = State::Exponent;
                    let _ = self.reader.next();
                    if let Ok(next) = self.reader.peek() {
                        if next == '-' || next == '+' {
                            if next == '-' {
                                exponent_sign = -1;
                            }
                        } else {
                            continue;
                        }
                    }
                }
                _ => break,
            }

            let _ = self.reader.next();
            c = self.reader.peek();
        }

        Ok(Token::Number(
            value * sign * 10f64.powi(exponent_value * exponent_sign),
        ))
    }

    pub fn read_boolean(&mut self) -> Result<Token, ReadError> {
        match self.reader.peek()? {
            't' if self.full_match("true".chars()) => Ok(Token::Boolean(true)),
            'f' if self.full_match("false".chars()) => Ok(Token::Boolean(false)),
            _ => Err(ReadError::IllegalToken),
        }
    }

    fn read_null(&mut self) -> Result<Token, ReadError> {
        if !self.full_match("null".chars()) {
            return Err(ReadError::IllegalToken);
        }
        Ok(Token::Null)
    }

    fn full_match(&mut self, str: Chars) -> bool {
        for c in str {
            match self.reader.next() {
                Ok(v) => {
                    if v != c {
                        return false;
                    }
                }
                Err(_) => return false,
            }
        }
        true
    }
}
