use std::str::Chars;

use crate::error::ReadError;
use crate::r#trait::Reader;

pub struct ByteReader<'a> {
    buffer: &'a [u8],
    pos: usize,
}
impl<'a> ByteReader<'a> {
    pub fn new(buffer: &'a [u8]) -> Self {
        Self { buffer, pos: 0 }
    }
}
impl<'a> Reader for ByteReader<'a> {
    type Item = u8;

    fn peek(&self) -> Result<Self::Item, ReadError> {
        self.buffer.get(self.pos).copied().ok_or(ReadError::Eof)
    }

    fn next(&mut self) -> Result<Self::Item, ReadError> {
        self.pos += 1;
        self.buffer.get(self.pos - 1).copied().ok_or(ReadError::Eof)
    }
}

pub struct CharReader<'a> {
    peeked: Option<char>,
    buffer: Chars<'a>,
}
impl<'a> CharReader<'a> {
    pub fn new(buffer: Chars<'a>) -> Self {
        Self {
            peeked: None,
            buffer,
        }
    }

    #[inline]
    pub fn peek(&mut self) -> Result<char, ReadError> {
        match self.peeked {
            None => {
                self.peeked = self.buffer.next();
                self.peeked.ok_or(ReadError::Eof)
            }
            Some(v) => Ok(v),
        }
    }

    #[inline]
    pub fn next(&mut self) -> Result<char, ReadError> {
        match self.peeked {
            None => self.buffer.next().ok_or(ReadError::Eof),
            Some(_) => {
                Ok(self.peeked.take().unwrap())
            }
        }
    }
}
