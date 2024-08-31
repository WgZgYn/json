use crate::error::ReadError;
use crate::r#trait::Reader;
use crate::token::Token;
use crate::value::{Map, Value};

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

impl<'a> Reader for TokenReader<'a> {
    type Item = &'a Token;

    fn peek(&mut self) -> Result<Self::Item, ReadError> {
        self.buffer.get(self.pos).ok_or(ReadError::Eof)
    }

    fn next(&mut self) -> Result<Self::Item, ReadError> {
        self.pos += 1;
        self.buffer.get(self.pos - 1).ok_or(ReadError::Eof)
    }
}

impl<'a> TokenReader<'a> {
    fn has_next(&mut self) -> bool {
        self.peek() != Err(ReadError::Eof) && self.peek() != Ok(&Token::Eof)
    }

    fn build_pair(&mut self) -> Result<(&String, Value), ReadError> {
        let v1 = self.next();
        let v2 = self.next();
        match (v1, v2) {
            (Ok(Token::String(s)), Ok(Token::Colon)) => match self.peek() {
                Ok(Token::BeginArray) => Ok((s, self.build_array().expect("a JsonArray"))),
                Ok(Token::BeginObject) => Ok((s, self.build_object().expect("a JsonObject"))),
                Ok(Token::Boolean(f)) => {
                    let _ = self.next();
                    Ok((s, Value::Boolean(*f)))
                }
                Ok(Token::Number(n)) => {
                    let _ = self.next();
                    Ok((s, Value::Number(*n)))
                }
                Ok(Token::Null) => {
                    let _ = self.next();
                    Ok((s, Value::Null))
                }
                Ok(Token::String(str)) => {
                    let _ = self.next();
                    Ok((s, Value::String(str.clone())))
                }
                Err(_) => Err(ReadError::Eof),
                _ => Err(ReadError::IllegalSyntax),
            },
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
                let mut value = Map::new();
                loop {
                    let temp = self.peek();
                    match (temp, &state) {
                        (Ok(Token::String(_)), &NextRead::Pair | &NextRead::PairOrEnd) => {
                            match self.build_pair() {
                                Ok((k, v)) => {
                                    value.insert(k.clone(), v);
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

    fn build_value(&mut self) -> Result<Value, ReadError> {
        let start = self.peek();
        match start {
            Ok(Token::String(s)) => {
                let _ = self.next();
                Ok(Value::String(s.clone()))
            }
            Ok(Token::Number(f)) => {
                let _ = self.next();
                Ok(Value::Number(*f))
            }
            Ok(Token::Null) => {
                let _ = self.next();
                Ok(Value::Null)
            }
            Ok(Token::Boolean(b)) => {
                let _ = self.next();
                Ok(Value::Boolean(*b))
            }
            Ok(Token::BeginArray) => self.build_array(),
            Ok(Token::BeginObject) => self.build_object(),
            Err(e) => Err(e),
            _ => Err(ReadError::IllegalSyntax),
        }
    }
}
