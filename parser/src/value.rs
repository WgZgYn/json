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

fn fmt(val: &Value, f: &mut Formatter<'_>) -> std::fmt::Result {
    match val {
        Value::String(s) => f.write_str(format!("\"{s}\"").as_str()),
        Value::Boolean(b) => f.write_str(format!("{b}").as_str()),
        Value::Number(n) => f.write_str(format!("{n}").as_str()),
        Value::Null => f.write_str("null"),
        Value::JsonObject(mp) => f.write_str(format!("{:#?}", mp).as_str()),
        Value::JsonArray(ar) => f.write_str(format!("{:#?}", ar).as_str()),
    }
}

impl Debug for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        fmt(self, f)
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        fmt(self, f)
    }
}

impl Index<&str> for Value {
    type Output = Value;
    fn index(&self, index: &str) -> &Self::Output {
        static NULL: Value = Value::Null;
        if let Value::JsonObject(mp) = self {
            mp.get(index).unwrap_or(&NULL)
        } else {
            &NULL
        }
    }
}

impl IndexMut<&str> for Value {
    fn index_mut(&mut self, index: &str) -> &mut Self::Output {
        static mut NULL: Value = Value::Null;
        unsafe {
            if let Value::JsonObject(mp) = self {
                mp.get_mut(index).unwrap_or(&mut NULL)
            } else {
                &mut NULL
            }
        }
    }
}

impl Index<usize> for Value {
    type Output = Value;
    fn index(&self, index: usize) -> &Self::Output {
        static NULL: Value = Value::Null;
        if let Value::JsonArray(mp) = self {
            mp.get(index).unwrap_or(&NULL)
        } else {
            &NULL
        }
    }
}

impl IndexMut<usize> for Value {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        unsafe {
            if let Value::JsonArray(mp) = self {
                mp.get_mut(index)
                    .expect("index mutability on not exist JsonArray index")
            } else {
                panic!("index mutability on not exist JsonArray index")
            }
        }
    }
}
