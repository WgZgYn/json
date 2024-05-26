#[derive(Debug, PartialEq)]
pub enum ReadError {
    IllegalToken,
    IllegalByte(u8),
    IllegalChar(char),
    Eof,
}