#[derive(Debug, PartialEq)]
pub enum ReadError {
    IllegalToken,
    IllegalChar(u8),
    Eof,
}