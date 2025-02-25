#![feature(test)]


extern crate test;

fn main() {
    //  TODO: refactor the value fmt, remove the tail comma
}

#[cfg(test)]
mod tests {
    /// cmd >
    /// cargo test --release -- --nocapture
    ///
    ///

    use std::collections::HashMap;
    use std::process::Termination;
    use serde_json::json;
    use json_parser::parse::{from_str, parse_str};
    use json_parser::{parser, tokenizer};

    const JSON: &'static str = include_str!("../../example.json"); // compile-time file
    const LARGE: &'static str = include_str!("../../data.json");
    #[test]
    fn index_map() {
        // the index_map is ordered it by the order by the index it inserted.
        let json =  parse_str::<parser::TokenStream<_>, tokenizer::CharTokenizer, _>(JSON).unwrap();
        println!("{json}");

        // the std map is none ordered or ordered by the key
        let mut  mp = HashMap::new();
        mp.insert(3, 4);
        mp.insert(1, 2);
        println!("{:#?}", mp);
    }


    use super::*;
    use test::Bencher;

    #[bench]
    fn time_bench_impl(b: &mut Bencher) -> impl Termination {
        // let mut sum0 = std::time::Duration::ZERO;
        // let mut sum1 = std::time::Duration::ZERO;
        //
        // for _ in 0..50 {
        //     let start = std::time::Instant::now();
        //     let a = from_str(JSON);
        //     let cost = start.elapsed();
        //     println!("parse_json cost: {:?}, {}", cost, a.is_ok());
        //     sum0 += cost;
        //
        //     let start = std::time::Instant::now();
        //     let a = serde_json::from_str::<serde_json::Value>(JSON);
        //     let cost = start.elapsed();
        //     println!("serde_json cost: {:?}, {}", cost, a.is_ok());
        //     sum1 += cost;
        // }
        //
        // println!("average: {:?}", sum0 / 50);
        // println!("average: {:?}", sum1 / 50);
        b.iter(|| from_str(LARGE));
    }
    #[bench]
    fn time_bench_serde(b: &mut Bencher) {
        b.iter(|| serde_json::from_str::<serde_json::Value>(LARGE));
    }

    #[test]
    fn test_json_macro() {
        let v = json!(
            {
                "a": 123,
                "array": [1, 2, 3],
                "bool": true,
                "object": {
                    "c": 45.156
                }
            }
        );

        println!("{}", serde_json::to_string_pretty(&v).unwrap());
    }
}