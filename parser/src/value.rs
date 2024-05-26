use std::collections::HashMap;

#[derive(Debug)]
pub enum Value {
    JsonObject(HashMap<String, Value>),
    JsonArray(Vec<Value>),
    String(String),
    Boolean(bool),
    Number(f64),
    Null,
}
