use crate::error::ReadError;
use crate::reader::TokenReader;


mod token;
mod value;
mod r#trait;
mod error;
mod reader;

fn main() {
    let mut reader = TokenReader::new(br#"{ "key": null, "val ": -123, "ok": true, "or": false }"#);
    let mut result = reader.read_token();
    while result != Err(ReadError::Eof) {
        println!("{:?}", result);
        result = reader.read_token();
    }
    println!("{:?}", result);
}
