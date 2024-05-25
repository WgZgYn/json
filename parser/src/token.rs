#[derive(Debug, PartialEq)]
pub enum Token {
    BeginObject,
    EndObject,
    BeginArray,
    EndArray,
    Colon,
    Comma,
    String(String),
    Number(f64),
    Null,
    Boolean(bool),
    WhiteSpace,
    Eof
}

pub enum BuildState {
    Object,
    Array,
    String,
    Number,
    Boolean,
    Null,
}