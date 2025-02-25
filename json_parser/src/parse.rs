use crate::error::ReadError;
use crate::parser::*;
use crate::r#trait::{DataHandler, TokenHandler, Tokenizer};
use crate::tokenizer::*;
use crate::value::Value;

pub fn parse_str_multi_char(json: &str) -> Result<Value, ReadError> {
    TokenOwner::with_tokenizer(MultiTokenizer::handle(json)).build_value()
}

pub fn parse_str_stream_char(json: &str) -> Result<Value, ReadError> {
    TokenStream::with_tokenizer(CharTokenizer::handle(json)).build_value()
}

pub fn parse_str_owner_char(json: &str) -> Result<Value, ReadError> {
    TokenOwner::with_tokenizer(CharTokenizer::handle(json)).build_value()
}

pub fn parse_str<T, R, U>(json: U) -> Result<Value, ReadError>
where
    T: TokenHandler<R>,
    R: DataHandler<U> + Tokenizer,
{
    T::with_tokenizer(R::handle(json)).parse()
}

pub fn from_str(s: &str) -> Result<Value, ReadError> {
    TokenStream::with_tokenizer(CharTokenizer::handle(s)).parse()
}
