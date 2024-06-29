use lazy_static::lazy_static;
use std::{collections::HashMap, iter::Peekable};

enum Json {
    Object(Vec<(String, Json)>),
    Array(Vec<Json>),
    String(String),
    Int(i32),
    Float(f64),
    Bool(bool),
    Null,
}

#[derive(Debug, Clone)]
enum Token {
    StrLit(String),
    NumLit(String),
    Lcurly,
    Rcurly,
    Lbrack,
    Rbrack,
    Colon,
    Comma,
}

lazy_static! {
    static ref SINGLE_CHAR_TOKENS: HashMap<char, Token> = HashMap::from([
        ('{', Token::Lcurly),
        ('}', Token::Rcurly),
        ('[', Token::Lbrack),
        (']', Token::Rbrack),
        (':', Token::Colon),
        (',', Token::Comma),
    ]);
}

fn read_str_lit<T: Iterator<Item = char>>(chars: &mut Peekable<T>) -> Result<Token, String> {
    let _ = chars.next(); // opening quotation mark
    let mut buf = String::new();
    while *chars
        .peek()
        .ok_or_else(|| String::from("Unclosed string literal"))?
        != '"'
    {
        buf.push(chars.next().unwrap());
    }
    let _ = chars.next(); // closing quotation mark
    Ok(Token::StrLit(buf))
}

fn read_num_lit<T: Iterator<Item = char>>(chars: &mut Peekable<T>) -> Result<Token, String> {
    let mut buf = String::new();
    loop {
        if let Some(c) = chars.peek() {
            if *c != '.' && !c.is_digit(10) {
                break;
            }
        } else {
            break;
        }
        buf.push(chars.next().unwrap());
    }
    Ok(Token::NumLit(buf))
}

fn tokenize(string: &String) -> Result<Vec<Token>, String> {
    let mut chars = string.chars().peekable();
    let mut tokens = Vec::<Token>::new();
    loop {
        loop {
            match chars.peek() {
                Some(c) => {
                    if !c.is_whitespace() {
                        break;
                    }
                }
                None => return Ok(tokens),
            }
            let _ = chars.next();
        }
        let c = *chars.peek().unwrap();
        if SINGLE_CHAR_TOKENS.contains_key(&c) {
            tokens.push(SINGLE_CHAR_TOKENS.get(&c).unwrap().clone());
            let _ = chars.next();
        } else if c == '"' {
            tokens.push(read_str_lit(&mut chars)?);
        } else if c == '.' || c.is_digit(10) {
            tokens.push(read_num_lit(&mut chars)?)
        } else {
            todo!()
        }
    }
}

// fn parse_raw<T: Iterator<Item = Token>>(chars: &mut T) -> Json {
//     todo!()
// }

fn main() {
    let json_str = String::from("{[\"the\", 709, .097, \"dar\"]}");
    dbg!(tokenize(&json_str));
}
