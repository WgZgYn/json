fn main() {
    //  TODO: refactor the value fmt, remove the tail comma
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use json_parser::parse::parse_str;
    use json_parser::{parser, tokenizer};

    const JSON: &'static str = include_str!("../../example.json"); // compile-time file
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

    #[test]
    fn time_bench() {
        let mut sum0 = std::time::Duration::ZERO;
        let mut sum1 = std::time::Duration::ZERO;

        for _ in 0..50 {
            let start = std::time::Instant::now();
            let a = parse_str::<parser::TokenStream<_>, tokenizer::CharTokenizer, &str>(JSON);
            let cost = start.elapsed();
            println!("serde_json cost: {:?}, {}", cost, a.is_ok());
            sum0 += cost;

            let start = std::time::Instant::now();
            let a = serde_json::from_str::<serde_json::Value>(JSON);
            let cost = start.elapsed();
            println!("parse_json cost: {:?}, {}", cost, a.is_ok());
            sum1 += cost;
        }

        println!("average: {:?}", sum0 / 50);
        println!("average: {:?}", sum1 / 50);
    }
}