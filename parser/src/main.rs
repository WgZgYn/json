#![allow(dead_code)]
#![allow(unused)]

use std::collections::HashMap;

use crate::token::fmt_print_tokens;
use crate::tokenizer::{Tokenizer, TokenReader};
use crate::value::Value;

mod token;
mod value;
mod r#trait;
mod error;
mod reader;
mod tokenizer;

fn main() {
    let start = std::time::Instant::now();
    let json = std::fs::read_to_string("/home/wzy/Documents/rust_pojects/json/data.json").unwrap();
    let mut reader = Tokenizer::new(json.as_str());
    reader.read_tokens();
    // fmt_print_tokens(&reader.tokens);
    println!("cost: {:?}", start.elapsed());
    println!();

    let sub = HashMap::from([("sub".to_string(), Value::Null), ("arr".to_string(), Value::JsonArray(vec![1., 2., 3.].into_iter().map(|v| Value::Number(v)).collect()))]);
    let mp = HashMap::from([("key".to_string(), Value::String("123".to_string())), ("val".to_string(), Value::Boolean(false)), ("inner".to_string(), Value::JsonObject(sub))]);
    let json = Value::JsonObject(mp);

    println!("{:#?}", json);
}
