use crate::error::ReadError;
use crate::r#trait::Reader;
use crate::token::Token;

struct CharReader<'a> {
    buffer: &'a [u8],
    pos: usize,
}

impl<'a> CharReader<'a> {
    fn new(buffer: &'a [u8]) -> Self {
        Self { buffer, pos: 0 }
    }
}

pub struct TokenReader<'a> {
    reader: CharReader<'a>,
    tokens: Vec<Token>,
}

impl<'a> TokenReader<'a> {
    pub fn new(buffer: &'a [u8]) -> Self {
        Self { reader: CharReader::new(buffer), tokens: Vec::new() }
    }

    pub fn read_token(&mut self) -> Result<Token, ReadError> {
        let char = self.reader.peek();
        match char {
            Ok(v) => {
                match v {
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
                            Err(ReadError::IllegalChar(c))
                        }
                    }
                }
            }
            Err(e) => Err(e)
        }
    }

    fn read_string(&mut self) -> Result<Token, ReadError> {
        let mut buffer = String::new();

        #[derive(PartialEq)]
        enum State {
            None,
            Begin,
            End,
        }

        let mut state = State::None;
        let mut escape = false;
        let mut c = self.reader.next();

        while state != State::End {
            match c {
                Err(e) => return Err(e),
                Ok(v) => {
                    match v {
                        b'"' => {
                            if escape {
                                buffer.push(char::from(v));
                                escape = false;
                            } else {
                                if state == State::None {
                                    state = State::Begin;
                                } else if state == State::Begin {
                                    state = State::End;
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
                    }
                }
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
        let mut c = self.reader.next();

        if let Ok(v) = c {
            if v == b'-' {
                sign = -1.;
            }
            c = self.reader.next();
        }

        loop {
            match c {
                Err(e) => break,
                Ok(v @ b'0'..=b'9') => {
                    if !point {
                        value = value * 10. + (v - b'0') as f64;
                    } else {
                        value += rate * (v - b'0') as f64;
                        rate *= 0.1;
                    }
                }
                Ok(b'.') => {
                    if !point { point = true; }
                }
                _ => break,
            }
            c = self.reader.next();
        }

        Ok(Token::Number(value * sign))
    }

    fn read_char_token(&mut self) -> Result<Token, ReadError> {
        match self.reader.next() {
            Ok(v) => {
                match v {
                    b':' => Ok(Token::Colon),
                    b',' => Ok(Token::Comma),
                    b'{' => Ok(Token::BeginObject),
                    b'}' => Ok(Token::EndObject),
                    b'[' => Ok(Token::BeginArray),
                    b']' => Ok(Token::EndArray),
                    c => Err(ReadError::IllegalChar(c))
                }
            }
            Err(e) => Err(e),
        }
    }

    pub fn read_boolean(&mut self) -> Result<Token, ReadError> {
        const TRUE: &[u8; 4] = b"true";
        const FALSE: &[u8; 5] = b"false";

        let f = self.reader.peek();
        if f.is_err() { return Err(f.err().unwrap()); }

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
                    if v != *c { return false; }
                }
                Err(_) => return false,
            }
        }
        true
    }
}

impl<'a> Reader for CharReader<'a> {
    type Item = u8;

    fn peek(&self) -> Result<Self::Item, ReadError> {
        self.buffer.get(self.pos).copied().ok_or(ReadError::Eof)
    }

    fn next(&mut self) -> Result<Self::Item, ReadError> {
        self.pos += 1;
        self.buffer.get(self.pos - 1).copied().ok_or(ReadError::Eof)
    }
}

