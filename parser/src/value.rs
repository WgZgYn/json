// #![allow(dead_code)]
// #![allow(unused)]

use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter};
use std::ops::{Index, IndexMut};

// #[derive(Debug)]
pub enum Value {
    JsonObject(HashMap<String, Value>),
    JsonArray(Vec<Value>),
    String(String),
    Boolean(bool),
    Number(f64),
    Null,
}

impl Debug for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::String(s) => f.write_str(format!("\"{s}\"").as_str()),
            Value::Boolean(b) => f.write_str(format!("{b}").as_str()),
            Value::Number(n) => f.write_str(format!("{n}").as_str()),
            Value::Null => f.write_str("null"),
            Value::JsonObject(mp) => f.write_str(format!("{:#?}", mp).as_str()),
            Value::JsonArray(ar) => f.write_str(format!("{:#?}", ar).as_str()),
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::String(s) => f.write_str(format!("\"{s}\"").as_str()),
            Value::Boolean(b) => f.write_str(format!("{b}").as_str()),
            Value::Number(n) => f.write_str(format!("{n}").as_str()),
            Value::Null => f.write_str("null"),
            Value::JsonObject(mp) => f.write_str(format!("{:#?}", mp).as_str()),
            Value::JsonArray(ar) => f.write_str(format!("{:#?}", ar).as_str()),
        }
    }
}

pub enum ValueType {
    JsonObject,
    JsonArray,
    String,
    Boolean,
    Number,
    Null,
}

impl<'a> Index<&'a str> for Value {
    type Output = &'a Value;

    fn index(&self, index: &str) -> &Self::Output {
        todo!()
    }
}

impl IndexMut<&str> for Value {
    fn index_mut(&mut self, index: &str) -> &mut Self::Output {
        todo!()
    }
}
