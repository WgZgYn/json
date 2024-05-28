#[derive(Debug, PartialEq)]
pub enum ReadError {
    IllegalEscape,
    IllegalSyntax,
    IllegalToken,
    IllegalByte(u8),
    IllegalChar(char),
    Eof,
}
