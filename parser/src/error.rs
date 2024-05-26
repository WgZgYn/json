#[derive(Debug, PartialEq)]
pub enum ReadError {
    IllegalSyntax,
    IllegalToken,
    IllegalByte(u8),
    IllegalChar(char),
    Eof,
}
