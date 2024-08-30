use crate::error::ReadError;
use crate::r#trait::{JsonHandler, Tokenizer};
use crate::token::Token;
use crate::tokenizer::char_tokenizer::CharTokenizer;
use rayon::prelude::*;
use std::collections::HashSet;

pub struct MultiTokenizer<'a> {
    data: &'a str,
}

impl<'a> MultiTokenizer<'a> {
    fn split(&self) -> (usize, usize, usize) {
        let bytes = self.data.as_bytes();
        let l = bytes.len() as i32 / 4;
        let i0 = Self::find(bytes, l);
        let i1 = Self::find(bytes, l * 2);
        let i2 = Self::find(bytes, l * 3);
        (i0, i1, i2)
    }
    fn find(bytes: &[u8], mut start: i32) -> usize {
        let mut inc = 1;
        let mut forawrd = 1;
        loop {
            match bytes[start as usize] {
                b'[' | b']' | b',' | b'{' | b'}' | b':' => return start as usize,
                _ => {
                    start += forawrd * inc;
                    inc += 1;
                    forawrd *= -1;
                    if start >= bytes.len() as i32 {
                        panic!()
                    }
                }
            }
        }
    }

    fn n_min(l: usize, result: &mut [usize], i: usize) {
        for (p, v) in result.iter_mut().enumerate() {
            if v.abs_diff(l * p + l) > i.abs_diff(l * p + l) {
                *v = i;
                break;
            }
        }
    }

    fn scan(&self, n: usize) -> Result<Vec<usize>, ReadError> {
        let char_token = HashSet::from([b'{', b'}', b':', b',', b'[', b']']);
        let bytes = self.data.as_bytes();
        let mut result = vec![0; n - 1];

        enum State {
            InQuote,
            Escape,
            None,
        }

        let mut state = State::None;
        for (i, &b) in bytes.into_iter().enumerate() {
            match state {
                State::None => match b {
                    b'"' => state = State::InQuote,
                    c => {
                        if c.is_ascii_whitespace() || char_token.contains(&c) {
                            Self::n_min(self.data.len() / n, &mut result, i)
                        }
                    }
                },
                State::InQuote => {
                    if b == b'"' {
                        state = State::None;
                    } else if b == b'\\' {
                        state = State::Escape;
                    }
                }
                State::Escape => state = State::InQuote,
            }
        }
        Ok(result)
    }
}

impl<'a> Tokenizer for MultiTokenizer<'a> {
    fn read_tokens(&mut self) -> Vec<Token> {
        // let start = std::time::Instant::now();
        let v = self.scan(4).unwrap();
        // println!("{:?}", start.elapsed());
        // self.strings = strings;

        // println!("{:?}", v);
        // let (a, b, c) = self.split();
        let v = vec![
            (0, v[0]),
            (v[0] + 1, v[1]),
            (v[1] + 1, v[2]),
            (v[2] + 1, self.data.len() - 1),
        ];
        // let v = vec![(0, a), (a + 1, b), (b + 1, c), (c + 1, self.data.len() - 1)];

        v.into_par_iter()
            .map(|v| CharTokenizer::new(&self.data[v.0..=v.1]).read_tokens())
            .flatten()
            .collect()
    }
}

impl<'a> JsonHandler<&'a str> for MultiTokenizer<'a> {
    fn new(data: &'a str) -> Self {
        Self { data }
    }
}
