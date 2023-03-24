use crate::token::{Token, TokenType};

pub fn scan(source: String) -> Vec<Token> {
    let mut i: i32 = 0;
    let mut tokens: Vec<Token> = Vec::new();
    
    while i < source.len() as i32 {

        let c = source.chars().nth(i as usize).unwrap();
        i += 1;

        tokens.push(match_token(c, i));
    };

    tokens
}

fn match_token(c: char, i: i32) -> Token {
    match c {
        '+' => Token {
            token_type: TokenType::Plus,
            index: i
        },
        '-' => Token {
            token_type: TokenType::Minus,
            index: i
        },
        '(' => Token {
            token_type: TokenType::LParen,
            index: i
        },
        ')' => Token {
            token_type: TokenType::RParen,
            index: i
        },
        '0' ..= '9' => Token {
            token_type: TokenType::Number(c.to_digit(10).unwrap() as i32),
            index: i
        },
        'a' ..= 'z' | 'A' ..= 'Z' => Token {
            token_type: TokenType::Identifier(c.to_string()),
            index: i
        },
        _ => Token {
            token_type: TokenType::Error(String::from("Unknown character")),
            index: i
        }
    }
}

