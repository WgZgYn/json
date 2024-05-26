#[derive(Debug, PartialEq)]
pub enum Token {
    BeginObject,
    EndObject,
    BeginArray,
    EndArray,
    Colon,
    Comma,
    String(String),
    Number(f64),
    Null,
    Boolean(bool),
    WhiteSpace,
    Eof
}

pub enum BuildState {
    Object,
    Array,
}

pub fn fmt_print_tokens(tokens: &[Token]) {
    let mut indent = 0;
    let mut newline = true;
    for token in tokens {
        match token {
            Token::BeginObject => {
                println!("{{");
                indent += 4;
                newline = true;
            }
            Token::EndObject => {
                indent -= 4;
                println!();
                print!("{}}}", " ".repeat(indent));
                newline = true;
            }
            Token::BeginArray => {
                println!("[");
                indent += 4;
                newline = true;
            }
            Token::EndArray => {
                indent -= 4;
                println!();
                print!("{}]", " ".repeat(indent));
                newline = true;
            }
            Token::Comma => {
                println!(", ");
                newline = true;
            }
            Token::Colon => {
                print!(": ");
            }
            Token::String(s) => {
                if newline {
                    print!("{}", " ".repeat(indent));
                    newline = false;
                }
                print!("\"{}\"", s);
            }
            Token::Number(s) => {
                print!("{}", s);
            }
            Token::Boolean(s) => {
                print!("{}", s);
            }
            Token::Null => {
                print!("null");
            }
            e => {
                if newline {
                    print!("{}", " ".repeat(indent));
                    newline = false;
                }
                if *e != Token::WhiteSpace {
                    print!("{:?}", e);
                }
            }
        }
    }
}