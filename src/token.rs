use std::fmt::Display;


pub enum TokenType {
    Number(i32),
    LParen,
    RParen,
    Plus,
    Minus,
    Identifier(String),
    Error(String)
}

pub struct Token {
    pub token_type: TokenType,
    pub index: i32
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let output = match(&self.token_type) {
            TokenType::Number(num) => "Number",
            TokenType::LParen => "LParen",
            TokenType::RParen => "RParen",
            TokenType::Plus => "Plus",
            TokenType::Minus => "Minus",
            TokenType::Identifier(String) => "Identifier",
            TokenType::Error(String) => "Error",
            _ => panic!("Token display did not recognize token_type.")
        };
        write!(f, "{}", output)
    }
}
