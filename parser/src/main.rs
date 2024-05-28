#![allow(warnings)]

use std::ops::{Add, Div};
use crate::parse::{parse_multi_thread, parse_str};
use crate::tokenizer::{MultiTokenizer, Tokenizer};

mod error;
mod parse;
mod reader;
mod token;
mod tokenizer;
mod r#trait;
mod value;

fn main() {
    // let json = std::fs::read_to_string("/home/wzy/Documents/rust_pojects/json/example.json").unwrap();// runtime file
    let json = include_str!("/home/wzy/Documents/rust_pojects/json/generated.json"); // compile-time file

    let mut sum = std::time::Duration::new(0, 0);
    for _ in 0..25  {
        let start = std::time::Instant::now();
        let mut a = parse_str(json);
        // let a = serde_json::from_str::<serde_json::Value>(json);
        let cost = start.elapsed();
        println!("cost: {:?}, {}", cost, a.is_ok());
        sum += cost;
    }
    println!("average: {:?}", sum / 25);


    // let start = std::time::Instant::now();
    // let mut b = parse_multi_thread(json);
    // println!("cost: {:?}", start.elapsed());

    // for i in 0..69 {
    // dbg!(&a.tokens[i], &v[i]);
    // }

    // assert_eq!(a, b);
}
