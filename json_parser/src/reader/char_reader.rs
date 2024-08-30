use crate::error::ReadError;
use crate::r#trait::Reader;
use std::str::Chars;

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
}

impl<'a> Reader for CharReader<'a> {
    type Item = char;

    #[inline]
    fn peek(&mut self) -> Result<Self::Item, ReadError> {
        match self.peeked {
            Some(v) => Ok(v),
            None => {
                self.peeked = self.buffer.next();
                self.peeked.ok_or(ReadError::Eof)
            }
        }
    }

    #[inline]
    fn next(&mut self) -> Result<Self::Item, ReadError> {
        match self.peeked {
            Some(_) => Ok(self.peeked.take().unwrap()),
            None => self.buffer.next().ok_or(ReadError::Eof),
        }
    }
}
