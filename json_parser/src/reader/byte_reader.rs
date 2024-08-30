use crate::error::ReadError;

pub struct ByteReader<'a> {
    buffer: &'a [u8],
    pos: usize,
}

impl<'a> ByteReader<'a> {
    pub fn new(buffer: &'a [u8]) -> Self {
        Self { buffer, pos: 0 }
    }
}

impl<'a> ByteReader<'a> {
    #[inline]
    pub fn peek(&mut self) -> Result<u8, ReadError> {
        self.buffer.get(self.pos).copied().ok_or(ReadError::Eof)
    }

    #[inline]
    pub fn next(&mut self) -> Result<u8, ReadError> {
        self.pos += 1;
        self.buffer.get(self.pos - 1).copied().ok_or(ReadError::Eof)
    }
}
