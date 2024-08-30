use crate::error::ReadError;
use crate::token::Token;
use crate::value::Value;

pub trait Reader {
    type Item;
    fn peek(&mut self) -> Result<Self::Item, ReadError>;
    fn next(&mut self) -> Result<Self::Item, ReadError>;
}

pub trait Tokenizer {
    fn read_tokens(&mut self) -> Vec<Token>;
}

pub trait StreamToken {
    fn read_token(&mut self) -> Result<Token, ReadError>;
}

impl<T: StreamToken> Tokenizer for T {
    fn read_tokens(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        let mut result = self.read_token();
        while result.is_ok() && result != Ok(Token::Eof) {
            tokens.push(result.unwrap());
            result = self.read_token();
        }
        tokens
    }
}

pub trait TokenHandler<T: Tokenizer> {
    fn new(tokenizer: T) -> Self;
    fn parse(&mut self) -> Result<Value, ReadError>;
}

pub trait JsonHandler<T> {
    fn new(j: T) -> Self;
}

trait JsonBuild {
    fn build_value(&mut self) -> Result<Value, ReadError>;
    fn build_array(&mut self) -> Result<Value, ReadError>;
    fn build_object(&mut self) -> Result<Value, ReadError>;
    fn build_pair(&mut self) -> Result<Value, ReadError>;
}