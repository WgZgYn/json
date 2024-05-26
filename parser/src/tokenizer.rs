#![allow(unused)]

use crate::error::ReadError;
use crate::r#trait::Reader;
use crate::reader::{ByteReader, CharReader};
use crate::token::Token;
use std::str::Chars;

pub struct ByteTokenizer<'a> {
    reader: ByteReader<'a>,
    pub tokens: Vec<Token>,
}

impl<'a> ByteTokenizer<'a> {
    pub fn new(buffer: &'a str) -> Self {
        Self {
            reader: ByteReader::new(buffer.as_bytes()),
            tokens: Vec::new(),
        }
    }
    pub fn read_token(&mut self) -> Result<Token, ReadError> {
        let char = self.reader.peek();
        match char {
            Ok(v) => match v {
                b'{' | b'}' | b':' | b',' | b'[' | b']' => self.read_char_token(),
                b'"' => self.read_string(),
                b'n' => self.read_null(),
                b't' | b'f' => self.read_boolean(),
                c if (c <= b'9' && c >= b'0') || c == b'-' => self.read_number(),
                c => {
                    let _ = self.reader.next();
                    if c.is_ascii_whitespace() {
                        Ok(Token::WhiteSpace)
                    } else {
                        Err(ReadError::IllegalByte(c))
                    }
                }
            },
            Err(ReadError::Eof) => Ok(Token::Eof),
            Err(e) => Err(e),
        }
    }

    pub fn read_tokens(&mut self) {
        let mut result = self.read_token();
        while result != Ok(Token::Eof) && result != Err(ReadError::Eof) {
            self.tokens.push(result.unwrap());
            result = self.read_token();
        }
    }

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
        let mut value = 0.;
        let mut sign = 1.;
        let mut point = false;
        let mut rate = 0.1;
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
                Ok(v @ b'0'..=b'9') => {
                    if !point {
                        value = value * 10. + (v - b'0') as f64;
                    } else {
                        value += rate * (v - b'0') as f64;
                        rate *= 0.1;
                    }
                }
                Ok(b'.') => {
                    if !point {
                        point = true;
                    }
                }
                _ => break,
            }

            let _ = self.reader.next();
            c = self.reader.peek();
        }

        Ok(Token::Number(value * sign))
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

    pub fn read_boolean(&mut self) -> Result<Token, ReadError> {
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

pub struct Tokenizer<'a> {
    reader: CharReader<'a>,
    pub tokens: Vec<Token>,
}

impl<'a> Tokenizer<'a> {
    pub fn new(buffer: &'a str) -> Self {
        Self {
            reader: CharReader::new(buffer.chars()),
            tokens: Vec::new(),
        }
    }

    fn read_token(&mut self) -> Result<Token, ReadError> {
        let char = self.reader.peek();
        match char {
            Ok(v) => match v {
                '{' | '}' | ':' | ',' | '[' | ']' => self.read_char_token(),
                '"' => self.read_string(),
                'n' => self.read_null(),
                't' | 'f' => self.read_boolean(),
                c if (c <= '9' && c >= '0') || c == '-' => self.read_number(),
                c => {
                    let _ = self.reader.next();
                    if c.is_ascii_whitespace() {
                        Ok(Token::WhiteSpace)
                    } else {
                        Err(ReadError::IllegalChar(c))
                    }
                }
            },
            Err(ReadError::Eof) => Ok(Token::Eof),
            Err(e) => Err(e),
        }
    }

    pub fn read_tokens(&mut self) {
        let mut result = self.read_token();
        while result != Ok(Token::Eof) && result != Err(ReadError::Eof) {
            if result != Ok(Token::WhiteSpace) {
                self.tokens.push(result.unwrap());
            }
            result = self.read_token();
        }
    }

    fn read_string(&mut self) -> Result<Token, ReadError> {
        let mut buffer = String::new();

        let mut start = false;
        let mut escape = false;
        let mut c = self.reader.next();

        loop {
            match c {
                Err(e) => return Err(e),
                Ok(v) => match v {
                    '"' => {
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
                                '/' => c = '/',
                                '\\' => c = '\\',
                                't' => c = '\t',
                                'n' => c = '\n',
                                'r' => c = '\r',
                                _ => (),
                            }
                            escape = false;
                            buffer.push(char::from(c));
                        } else if c == '\\' {
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
        let mut value = 0.;
        let mut sign = 1.;
        let mut point = false;
        let mut rate = 0.1;
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
                Ok(v @ '0'..='9') => {
                    if !point {
                        value = value * 10. + (v as u32 - '0' as u32) as f64;
                    } else {
                        value += rate * (v as u32 - '0' as u32) as f64;
                        rate *= 0.1;
                    }
                }
                Ok('.') => {
                    if !point {
                        point = true;
                    }
                }
                _ => break,
            }

            let _ = self.reader.next();
            c = self.reader.peek();
        }

        Ok(Token::Number(value * sign))
    }

    fn read_char_token(&mut self) -> Result<Token, ReadError> {
        match self.reader.next() {
            Ok(v) => match v {
                ':' => Ok(Token::Colon),
                ',' => Ok(Token::Comma),
                '{' => Ok(Token::BeginObject),
                '}' => Ok(Token::EndObject),
                '[' => Ok(Token::BeginArray),
                ']' => Ok(Token::EndArray),
                c => Err(ReadError::IllegalChar(c)),
            },
            Err(e) => Err(e),
        }
    }

    pub fn read_boolean(&mut self) -> Result<Token, ReadError> {
        const TRUE: &str = "true";
        const FALSE: &str = "false";

        let f = self.reader.peek();
        if f.is_err() {
            return Err(f.err().unwrap());
        }

        if f.unwrap() == 't' {
            if self.full_match(TRUE.chars()) {
                return Ok(Token::Boolean(true));
            }
        } else {
            if self.full_match(FALSE.chars()) {
                return Ok(Token::Boolean(false));
            }
        }

        Err(ReadError::IllegalToken)
    }

    fn read_null(&mut self) -> Result<Token, ReadError> {
        const NULL: &str = "null";
        if !self.full_match(NULL.chars()) {
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
