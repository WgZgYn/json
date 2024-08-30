use crate::error::ReadError;
use crate::parser::*;
use crate::r#trait::{JsonHandler, TokenHandler, Tokenizer};
use crate::tokenizer::*;
use crate::value::Value;

pub fn parse_str_multi_char(json: &str) -> Result<Value, ReadError> {
    TokenOwner::new(MultiTokenizer::new(json)).build_value()
}

pub fn parse_str_stream_char(json: &str) -> Result<Value, ReadError> {
    TokenStream::new(CharTokenizer::new(json)).build_value()
}

pub fn parse_str_owner_char(json: &str) -> Result<Value, ReadError> {
    TokenOwner::new(CharTokenizer::new(json)).build_value()
}

pub fn parse_str<T, R, U>(json: U) -> Result<Value, ReadError>
where
    T: TokenHandler<R>,
    R: JsonHandler<U> + Tokenizer,
{
    T::new(R::new(json)).parse()
}
