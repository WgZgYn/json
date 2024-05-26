use crate::token::fmt_print_tokens;
use crate::tokenizer::Tokenizer;

mod error;
mod parse;
mod reader;
mod token;
mod tokenizer;
mod r#trait;
mod value;

fn main() {
    let start = std::time::Instant::now();
    // let json = std::fs::read_to_string("/home/wzy/Documents/rust_pojects/json/data.json").unwrap();

    let json = r#"
{
    "glossary": {
        "title": "example glossary",
		"GlossDiv": {
            "title": "S",
			"GlossList": {
                "GlossEntry": {
                    "ID": "SGML",
					"SortAs": "SGML",
					"GlossTerm": "Standard Generalized Markup Language",
					"Acronym": "SGML",
					"Abbrev": "ISO 8879:1986",
					"GlossDef": {
                        "para": "A meta-markup language, used to create markup languages such as DocBook.",
						"GlossSeeAlso": ["GML", "XML"]
                    },
					"GlossSee": "markup"
                }
            }
        }
    }
}
"#;

    let mut reader = Tokenizer::new(json);
    reader.read_tokens();
    // fmt_print_tokens(&reader.tokens);
    println!("\ncost: {:?}", start.elapsed());
    let result = reader.parse();
    println!("{}", result.unwrap());

    // let sub = HashMap::from([
    //     ("sub".to_string(), Value::Null),
    //     (
    //         "arr".to_string(),
    //         Value::JsonArray(
    //             vec![1., 2., 3.]
    //                 .into_iter()
    //                 .map(|v| Value::Number(v))
    //                 .collect(),
    //         ),
    //     ),
    // ]);
    // let mp = HashMap::from([
    //     ("key".to_string(), Value::String("123".to_string())),
    //     ("val".to_string(), Value::Boolean(false)),
    //     ("inner".to_string(), Value::JsonObject(sub)),
    // ]);
    // let json = Value::JsonObject(mp);
    //
    // println!("{:#?}", json);
}
