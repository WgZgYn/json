use std::cell::RefCell;
use std::iter::Peekable;
use std::str::Chars;
use crate::error::ReadError;
use crate::r#trait::Reader;
use crate::token::{fmt_print_tokens, Token};

pub struct ByteReader<'a> {
    buffer: &'a [u8],
    pos: usize,
}

impl<'a> ByteReader<'a> {
    pub(crate) fn new(buffer: &'a [u8]) -> Self {
        Self { buffer, pos: 0 }
    }
}

pub struct CharReader<'a> {
    buffer: RefCell<Peekable<Chars<'a>>>,
    pos: usize,
}

impl<'a> CharReader<'a> {
    pub(crate) fn new(buffer: Chars<'a>) -> Self {
        Self { buffer: RefCell::new(buffer.peekable()), pos: 0 }
    }
}

impl<'a> Reader for CharReader<'a> {
    type Item = char;

    fn peek(&self) -> Result<Self::Item, ReadError> {
        self.buffer.borrow_mut().peek().copied().ok_or(ReadError::Eof)
    }

    fn next(&mut self) -> Result<Self::Item, ReadError> {
        self.buffer.borrow_mut().next().ok_or(ReadError::Eof)
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

