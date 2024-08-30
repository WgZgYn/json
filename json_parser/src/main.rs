use json_parser::*;
use parse::parse_str;


fn main() {
    let json = &std::fs::read_to_string("./generated.json").unwrap();// runtime file
    // let json = include_str!("../../generated.json"); // compile-time file

    let mut sum0 = std::time::Duration::new(0, 0);
    let mut sum1 = std::time::Duration::new(0, 0);

    for _ in 0..50 {
        let start = std::time::Instant::now();
        let a = parse_str::<parser::TokenStream<_>, tokenizer::ByteTokenizer, &str>(json);
        // let a = parse_str_stream_char(json);
        let cost = start.elapsed();
        println!("serde_json cost: {:?}, {}", cost, a.is_ok());
        sum0 += cost;

        let start = std::time::Instant::now();
        let a = serde_json::from_str::<serde_json::Value>(json);
        let cost = start.elapsed();
        println!("parse_json cost: {:?}, {}", cost, a.is_ok());
        sum1 += cost;
    }

    println!("average: {:?}", sum0 / 50);
    println!("average: {:?}", sum1 / 50);

}
