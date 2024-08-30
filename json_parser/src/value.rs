use std::collections::BTreeMap;
use std::fmt::{Debug, Display, Formatter};
use std::ops::{Index, IndexMut};

// #[derive(Debug)]
pub enum Value {
    JsonObject(BTreeMap<String, Value>),
    JsonArray(Vec<Value>),
    String(String),
    Boolean(bool),
    Number(f64),
    Null,
}

fn fmt(val: &Value, f: &mut Formatter<'_>) -> std::fmt::Result {
    match val {
        Value::String(s) => write!(f, "\"{}\"", s),
        Value::JsonObject(mp) => write!(f, "{:#?}", mp),
        Value::JsonArray(ar) => write!(f, "{:#?}", ar),
        Value::Number(n) => write!(f, "{}", n),
        Value::Boolean(b) => write!(f, "{}", b),
        Value::Null => write!(f, "null"),
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
        if let Value::JsonObject(mp) = self {
            mp.get_mut(index)
                .expect("mut index on not exist JsonObject index")
        } else {
            panic!("mut index on not JsonObject")
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
        if let Value::JsonArray(mp) = self {
            mp.get_mut(index)
                .expect("index mutability on not exist JsonArray index")
        } else {
            panic!("index mutability on not exist JsonArray index")
        }
    }
}
