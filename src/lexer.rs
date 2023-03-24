#![allow(warnings, unused)]

use crate::token::{Token, TokenType};
use regex::Regex;

pub fn scan(source: &String) -> Vec<Token> {
    let mut i: i32 = 0;
    let mut tokens: Vec<Token> = Vec::new();

    while i < (source.len() as i32) {
        let c = source.chars().nth(i as usize).unwrap();
        let token = match_token(&source, c, i);
        i = token.end;
        tokens.push(token);
    }

    tokens
}

fn match_token(source: &String, c: char, i: i32) -> Token {
    match c {
        'a'..='z' | 'A'..='Z' => {
            let str = scan_long_token(source, Regex::new("[a-zA-Z_]").unwrap(), i);
            let token_length = str.len();
            Token {
                token_type: TokenType::Identifier(str),
                start: i,
                end: i + token_length as i32,
            }
        },
        '0'..='9' => {
            let str = scan_long_token(source, Regex::new("[0-9]").unwrap(), i);
            let token_length = str.len();
            Token {
                token_type: TokenType::Number(str.parse().unwrap()),
                start: i,
                end: i + token_length as i32,
            }
        },
        ' ' => Token {
            token_type: TokenType::Space,
            start: i,
            end: i + 1,
        },
        '\n' => Token {
            token_type: TokenType::Newline,
            start: i,
            end: i + 1,
        },
        _ => Token {
            token_type: TokenType::Error(String::from("Unknown character")),
            start: i,
            end: i + 1,
        },
    }
}

// Scans a longer token like identifier or a number.
fn scan_long_token(source: &String, regex: Regex, mut i: i32) -> String {
    let start = i;
    let mut buffer = String::from("");
    loop {
        let c = source.chars().nth(i as usize).unwrap();
        buffer.push(c);
        if (i == (source.len() - 1) as i32) {
            break;
        }
        let next = source.chars().nth((i + 1) as usize).unwrap();
        i += 1;
        if (!regex.is_match(&String::from(next))) {
            break;
        }
    }

    buffer 
}

fn make_token(token_type: TokenType, start: i32, length: i32) -> Token {
    Token {
        token_type,
        start,
        end: start + length,
    }
}
