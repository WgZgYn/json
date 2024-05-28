use std::collections::HashMap;

use crate::error::ReadError;
use crate::r#trait::Reader;
use crate::token::Token;
use crate::token::{TokenOwner, TokenReader};
use crate::tokenizer::{ByteTokenizer, MultiTokenizer, Tokenizer};
use crate::value::Value;

impl<'a> TokenReader<'a> {
    fn has_next(&self) -> bool {
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
                let mut value: HashMap<String, Value> = HashMap::new();
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

impl TokenOwner {
    fn has_next(&self) -> bool {
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
                let mut value: HashMap<String, Value> = HashMap::new();
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

    fn build_value(&mut self) -> Result<Value, ReadError> {
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

impl<'a> ByteTokenizer<'a> {
    pub fn parse_once(self) -> Result<Value, ReadError> {
        let mut rd = TokenOwner::new(self.tokens);
        let val = rd.build_value();
        if rd.has_next() {
            Err(ReadError::IllegalSyntax)
        } else {
            val
        }
    }
}

impl<'a> Tokenizer<'a> {
    pub fn parse(&self) -> Result<Value, ReadError> {
        let mut rd = TokenReader::new(&self.tokens);
        let val = rd.build_value();
        if rd.has_next() {
            Err(ReadError::IllegalSyntax)
        } else {
            val
        }
    }

    pub fn parse_once(self) -> Result<Value, ReadError> {
        let mut rd = TokenOwner::new(self.tokens);
        let val = rd.build_value();
        if rd.has_next() {
            Err(ReadError::IllegalSyntax)
        } else {
            val
        }
    }
}

pub fn parse_str(json: &str) -> Result<Value, ReadError> {
    let mut t = Tokenizer::new(json);
    t.read_tokens();
    t.parse_once()
}

pub fn parse_multi_thread(json: &str) -> Result<Value, ReadError> {
    let mut t = MultiTokenizer::new(json);
    let mut rd = TokenOwner::new(t.read_tokens());
    let val = rd.build_value();
    if rd.has_next() {
        Err(ReadError::IllegalSyntax)
    } else {
        val
    }
}
