use std::fmt::{Display, Formatter};

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
    Eof,
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::BeginObject => f.write_str("{"),
            Token::EndObject => f.write_str("}"),
            Token::BeginArray => f.write_str("["),
            Token::EndArray => f.write_str("]"),
            Token::Colon => f.write_str(":"),
            Token::Comma => f.write_str(","),
            Token::String(s) => f.write_fmt(format_args!("\"{}\"", s)),
            Token::Number(s) => f.write_fmt(format_args!("{}", s)),
            Token::Null => f.write_str("null"),
            Token::Boolean(s) => f.write_fmt(format_args!("{}", s)),
            Token::WhiteSpace => Ok(()),
            Token::Eof => Ok(()),
        }
    }
}

pub fn fmt_print_tokens(tokens: &[Token]) {
    let mut indent = 0;
    let mut newline = true;
    for token in tokens {
        match token {
            Token::BeginArray => {
                println!("{token}");
                indent += 4;
                newline = true;
            }

            Token::BeginObject => {
                if newline {
                    print!("{}", " ".repeat(indent));
                }
                println!("{token}");
                indent += 4;
                newline = true;
            }

            Token::EndObject | Token::EndArray => {
                indent -= 4;
                println!();
                print!("{}{token}", " ".repeat(indent));
                newline = true;
            }
            Token::Comma => {
                println!(", ");
                newline = true;
            }
            Token::Colon => {
                print!(": ");
            }
            e => {
                if newline {
                    print!("{}", " ".repeat(indent));
                    newline = false;
                }
                print!("{e}");
            }
        }
    }
}
