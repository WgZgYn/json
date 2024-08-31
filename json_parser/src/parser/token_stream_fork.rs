use crate::error::ReadError;
use crate::r#trait::{StreamToken, TokenHandler};
use crate::token::Token;
use crate::value::{Map, Value};

pub struct TokenStream1<T: StreamToken> {
    iter: T,
    peeked: Option<Token>,
}

impl<T: StreamToken> TokenStream1<T> {
    fn peek(&mut self) -> Result<&Token, ReadError> {
        match self.peeked {
            Some(ref v) => Ok(v),
            None => {
                self.peeked = self.iter.read_token().ok();
                self.peeked.as_ref().ok_or(ReadError::Eof)
            }
        }
    }

    fn next(&mut self) -> Result<Token, ReadError> {
        match self.peeked {
            None => self.iter.read_token(),
            Some(_) => Ok(self.peeked.take().unwrap()),
        }
    }
}

impl<T: StreamToken> TokenStream1<T> {
    pub fn has_next(&mut self) -> bool {
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
        #[derive(Debug)]
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
                        e => {
                            println!("Error detected {:?}", e);
                            return Err(ReadError::IllegalSyntax);
                        }
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
