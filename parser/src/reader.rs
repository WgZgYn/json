#[allow(unused)]
#[allow(dead_code)]

use std::cell::RefCell;
use std::iter::Peekable;
use std::str::Chars;

use crate::error::ReadError;
use crate::r#trait::Reader;
use crate::token::Token;

pub struct ByteReader<'a> {
    buffer: &'a [u8],
    pos: usize,
}

impl<'a> ByteReader<'a> {
    pub fn new(buffer: &'a [u8]) -> Self {
        Self { buffer, pos: 0 }
    }
}

pub struct CharReader<'a> {
    buffer: RefCell<Peekable<Chars<'a>>>,
}

impl<'a> CharReader<'a> {
    pub fn new(buffer: Chars<'a>) -> Self {
        Self {
            buffer: RefCell::new(buffer.peekable()),
        }
    }
}

pub struct TokenReader<'a> {
    buffer: &'a [Token],
    pos: usize,
}

impl<'a> TokenReader<'a> {
    pub fn new(tokens: &'a [Token]) -> Self {
        Self {
            buffer: tokens,
            pos: 0,
        }
    }
}

impl<'a> Reader for TokenReader<'a> {
    type Item = &'a Token;

    fn peek(&self) -> Result<Self::Item, ReadError> {
        self.buffer.get(self.pos).ok_or(ReadError::Eof)
    }

    fn next(&mut self) -> Result<Self::Item, ReadError> {
        self.pos += 1;
        self.buffer.get(self.pos - 1).ok_or(ReadError::Eof)
    }
}

impl<'a> Reader for CharReader<'a> {
    type Item = char;

    fn peek(&self) -> Result<Self::Item, ReadError> {
        self.buffer
            .borrow_mut()
            .peek()
            .copied()
            .ok_or(ReadError::Eof)
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
