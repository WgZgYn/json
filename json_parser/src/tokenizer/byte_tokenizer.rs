use crate::error::ReadError;
use crate::r#trait::{JsonHandler, StreamToken, Tokenizer};
use crate::reader::ByteReader;
use crate::token::Token;

pub struct ByteTokenizer<'a> {
    reader: ByteReader<'a>,
}

impl<'a> StreamToken for ByteTokenizer<'a> {
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
                b'{' | b'}' | b':' | b',' | b'[' | b']' => self.read_char_token(),
                b'"' => self.read_string(),
                b'n' => self.read_null(),
                b't' | b'f' => self.read_boolean(),
                c if c.is_ascii_digit() || c == b'-' => self.read_number(),
                c => Err(ReadError::IllegalByte(c)),
            },
            Err(ReadError::Eof) => Ok(Token::Eof),
            Err(e) => Err(e),
        }
    }
}
impl<'a> JsonHandler<&'a str> for ByteTokenizer<'a> {
    fn new(buffer: &'a str) -> Self {
        Self {
            reader: ByteReader::new(buffer.as_bytes()),
        }
    }
}

impl<'a> ByteTokenizer<'a> {
    fn read_string(&mut self) -> Result<Token, ReadError> {
        let mut buffer = String::new();

        let mut start = false;
        let mut escape = false;
        let mut c = self.reader.next();

        loop {
            match c {
                Err(e) => return Err(e),
                Ok(v) => match v {
                    b'"' => {
                        if escape {
                            buffer.push(char::from(v));
                            escape = false;
                        } else {
                            if !start {
                                start = true;
                            } else {
                                break;
                            }
                        }
                    }
                    mut c => {
                        if escape {
                            match c {
                                b'/' => c = b'/',
                                b'\\' => c = b'\\',
                                b't' => c = b'\t',
                                b'n' => c = b'\n',
                                b'r' => c = b'\r',
                                _ => (),
                            }
                            escape = false;
                            buffer.push(char::from(c));
                        } else if c == b'\\' {
                            escape = true;
                        } else {
                            buffer.push(char::from(c));
                        }
                    }
                },
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
            if v == b'-' {
                sign = -1.;
                let _ = self.reader.next();
                c = self.reader.peek();
            }
        }

        loop {
            match c {
                Err(_) => break,
                Ok(v @ b'0'..=b'9') => match state {
                    State::Integer => value = value * 10. + (v - b'0') as f64,
                    State::Fraction => {
                        value += rate * (v as u32 - '0' as u32) as f64;
                        rate *= 0.1;
                    }
                    State::Exponent => {
                        exponent_value = exponent_value * 10 + (v as i32 - '0' as i32)
                    }
                },
                Ok(b'.') => {
                    if let State::Integer = state {
                        state = State::Fraction;
                    } else {
                        return Err(ReadError::IllegalToken);
                    }
                }
                Ok(b'e' | b'E') => {
                    if let State::Exponent = state {
                        return Err(ReadError::IllegalToken);
                    }
                    state = State::Exponent;
                    let _ = self.reader.next();
                    if let Ok(next) = self.reader.peek() {
                        if next == b'-' || next == b'+' {
                            if next == b'-' {
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
    fn read_char_token(&mut self) -> Result<Token, ReadError> {
        match self.reader.next() {
            Ok(v) => match v {
                b':' => Ok(Token::Colon),
                b',' => Ok(Token::Comma),
                b'{' => Ok(Token::BeginObject),
                b'}' => Ok(Token::EndObject),
                b'[' => Ok(Token::BeginArray),
                b']' => Ok(Token::EndArray),
                c => Err(ReadError::IllegalByte(c)),
            },
            Err(e) => Err(e),
        }
    }

    fn read_boolean(&mut self) -> Result<Token, ReadError> {
        const TRUE: &[u8; 4] = b"true";
        const FALSE: &[u8; 5] = b"false";

        let f = self.reader.peek();
        if f.is_err() {
            return Err(f.err().unwrap());
        }

        if f.unwrap() == b't' {
            if self.full_match(TRUE) {
                return Ok(Token::Boolean(true));
            }
        } else {
            if self.full_match(FALSE) {
                return Ok(Token::Boolean(false));
            }
        }

        Err(ReadError::IllegalToken)
    }

    fn read_null(&mut self) -> Result<Token, ReadError> {
        const NULL: &[u8; 4] = b"null";
        if !self.full_match(NULL) {
            return Err(ReadError::IllegalToken);
        }
        Ok(Token::Null)
    }

    fn full_match(&mut self, str: &[u8]) -> bool {
        for c in str {
            match self.reader.next() {
                Ok(v) => {
                    if v != *c {
                        return false;
                    }
                }
                Err(_) => return false,
            }
        }
        true
    }
}
