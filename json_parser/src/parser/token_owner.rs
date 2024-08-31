use crate::error::ReadError;
use crate::r#trait::{TokenHandler, Tokenizer};
use crate::token::Token;
use crate::value::{Map, Value};

pub struct TokenOwner {
    buffer: Vec<Token>,
    pos: usize,
}

impl<T: Tokenizer> TokenHandler<T> for TokenOwner {
    fn new(mut tokenizer: T) -> Self {
        Self {
            buffer: tokenizer.read_tokens(),
            pos: 0,
        }
    }

    fn parse(&mut self) -> Result<Value, ReadError> {
        self.build_value()
    }
}

impl TokenOwner {
    pub fn peek(&self) -> Result<&Token, ReadError> {
        self.buffer.get(self.pos).ok_or(ReadError::Eof)
    }

    pub fn next(&mut self) -> Result<Token, ReadError> {
        if self.pos >= self.buffer.len() {
            return Err(ReadError::Eof);
        }
        self.pos += 1;
        Ok(std::mem::replace(
            &mut self.buffer[self.pos - 1],
            Token::Null,
        ))
    }
}

impl TokenOwner {
    pub(crate) fn has_next(&self) -> bool {
        self.peek() != Err(ReadError::Eof) && self.peek() != Ok(&Token::Eof)
    }

    fn build_pair(&mut self) -> Result<(String, Value), ReadError> {
        let Token::String(key) = self.next().unwrap() else {
            panic!("can't be reached")
        };
        let Token::Colon = self.next()? else {
            panic!("expected a colon")
        };
        match self.peek() {
            Ok(Token::BeginArray) => Ok((key, self.build_array().expect("a JsonArray"))),
            Ok(Token::BeginObject) => Ok((key, self.build_object().expect("a JsonObject"))),
            Ok(Token::Boolean(_)) => {
                let Token::Boolean(f) = self.next().unwrap() else {
                    panic!()
                };
                Ok((key, Value::Boolean(f)))
            }
            Ok(Token::Number(_)) => {
                let Token::Number(n) = self.next().unwrap() else {
                    panic!()
                };
                Ok((key, Value::Number(n)))
            }
            Ok(Token::Null) => {
                let _ = self.next();
                Ok((key, Value::Null))
            }
            Ok(Token::String(_)) => {
                let Token::String(str) = self.next().unwrap() else {
                    panic!()
                };
                Ok((key, Value::String(str)))
            }
            // Err(_) => Err(ReadError::Eof),
            _ => Err(ReadError::IllegalSyntax),
        }
    }

    fn build_object(&mut self) -> Result<Value, ReadError> {
        enum NextRead {
            PairOrEnd,
            CommaOrEnd,
            Pair,
        }

        let start = self.next();
        match start {
            Ok(Token::BeginObject) => {
                let mut state = NextRead::PairOrEnd;
                let mut value: Map = Map::new();
                loop {
                    let temp = self.peek();
                    match (temp, &state) {
                        (Ok(Token::String(_)), &NextRead::Pair | &NextRead::PairOrEnd) => {
                            match self.build_pair() {
                                Ok((k, v)) => {
                                    value.insert(k, v);
                                }
                                Err(e) => return Err(e),
                            }
                            state = NextRead::CommaOrEnd;
                        }
                        (Ok(Token::Comma), &NextRead::CommaOrEnd) => {
                            let _ = self.next();
                            state = NextRead::Pair;
                        }
                        (Ok(Token::EndObject), &NextRead::PairOrEnd | &NextRead::CommaOrEnd) => {
                            let _ = self.next();
                            return Ok(Value::JsonObject(value));
                        }
                        (Err(e), _) => return Err(e),
                        _ => return Err(ReadError::IllegalSyntax),
                    }
                }
            }
            Err(e) => Err(e),
            _ => Err(ReadError::IllegalSyntax),
        }
    }

    fn build_array(&mut self) -> Result<Value, ReadError> {
        enum NextRead {
            ValueOrEnd,
            CommaOrEnd,
            Value,
        }

        let start = self.next();
        match start {
            Ok(Token::BeginArray) => {
                let mut array: Vec<Value> = Vec::new();
                let mut state = NextRead::ValueOrEnd;
                loop {
                    let temp = self.peek();
                    match (temp, &state) {
                        (Ok(Token::EndArray), &NextRead::ValueOrEnd | &NextRead::CommaOrEnd) => {
                            let _ = self.next();
                            return Ok(Value::JsonArray(array));
                        }

                        (Ok(Token::Comma), &NextRead::CommaOrEnd) => {
                            state = NextRead::Value;
                            let _ = self.next();
                        }

                        (
                            Ok(
                                Token::BeginObject
                                | Token::BeginArray
                                | Token::Null
                                | Token::Number(_)
                                | Token::String(_)
                                | Token::Boolean(_),
                            ),
                            &NextRead::Value | &NextRead::ValueOrEnd,
                        ) => {
                            match self.build_value() {
                                Ok(v) => array.push(v),
                                Err(e) => return Err(e),
                            }
                            state = NextRead::CommaOrEnd;
                        }

                        (Err(e), _) => return Err(e),
                        _ => return Err(ReadError::IllegalSyntax),
                    }
                }
            }
            Err(e) => Err(e),
            _ => Err(ReadError::IllegalSyntax),
        }
    }

    pub fn build_value(&mut self) -> Result<Value, ReadError> {
        let start = self.peek();
        match start {
            Ok(Token::String(_)) => {
                let Token::String(s) = self.next().unwrap() else {
                    panic!()
                };
                Ok(Value::String(s))
            }
            Ok(Token::Number(_)) => {
                let Token::Number(f) = self.next().unwrap() else {
                    panic!()
                };
                Ok(Value::Number(f))
            }
            Ok(Token::Null) => {
                let _ = self.next();
                Ok(Value::Null)
            }
            Ok(Token::Boolean(_)) => {
                let Token::Boolean(b) = self.next().unwrap() else {
                    panic!()
                };
                Ok(Value::Boolean(b))
            }
            Ok(Token::BeginArray) => self.build_array(),
            Ok(Token::BeginObject) => self.build_object(),
            Err(e) => Err(e),
            _ => Err(ReadError::IllegalSyntax),
        }
    }
}
