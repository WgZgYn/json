use crate::error::ReadError;

pub trait Reader {
    type Item;
    fn peek(&self) -> Result<Self::Item, ReadError>;
    fn next(&mut self) -> Result<Self::Item, ReadError>;
}
