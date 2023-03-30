use std::fmt::Display;

use super::token::{Token, TokenType};

#[allow(unused)]
pub struct PrintNode {
    pub expr: i32,
}

pub struct ErrorNode {
    msg: String,
    start: i32,
    end: i32,
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let output = match &self {
            Node::Print(_) => "PrintNode",
            Node::Root(_) => "RootNode",
            _ => panic!("Token display did not recognize token_type."),
        };
        write!(f, "{}", output)
    }
}

pub enum Node {
    Print(PrintNode),
    Root(super::RootNode),
    Number(i32),
    Error(ErrorNode),
}

pub struct Parser {
    pub current_token: Option<Token>,
    pub tokens: Vec<Token>,
    pub program_root: Node,
    pub token_index: usize,
}

impl Parser {
    // Run parsing, call from outside
    pub fn parse(&mut self) -> super::RootNode {
        let mut root = super::RootNode { stmts: Vec::new() };
        loop {
            self.next_token();
            if let Some(_token) = &self.current_token {
                println!("{}", _token);
                match self.current_token.clone().unwrap().token_type {
                    TokenType::Identifier(identifier) => {
                        root.stmts.push(self.match_identifier(&identifier))
                    }
                    TokenType::Error(msg) => root.stmts.push(Node::Error(ErrorNode {
                        msg,
                        start: self.token_index as i32,
                        end: self.token_index as i32,
                    })),
                    TokenType::Number(_) => root.stmts.push(Node::Error(ErrorNode {
                        msg: "Unexpected token: Number".to_string(),
                        start: self.token_index as i32,
                        end: self.token_index as i32,
                    })),
                    _ => (),
                };
            } else {
                break;
            }
        }

        root
    }

    fn match_identifier(&mut self, identifier: &str) -> Node {
        match identifier {
            "print" => self.match_print(),
            _ => Node::Error(ErrorNode {
                msg: "Unexpected token".to_string(),
                start: self.current_token.as_ref().unwrap().start,
                end: self.current_token.as_ref().unwrap().end,
            }),
        }
    }

    fn match_print(&mut self) -> Node {
        self.next_token();
        if let TokenType::Number(num) = self.current_token.as_ref().unwrap().token_type {
            Node::Print(PrintNode { expr: num })
        } else {
            Node::Error(ErrorNode {
                msg: "Unexpected token, expected expression".to_string(),
                start: self.token_index as i32,
                end: self.token_index as i32,
            })
        }
    }

    // Set the current_token to Some(Token) or None if EOF. Whitespaces are skipped
    fn next_token(&mut self) {
        while self.token_index < self.tokens.len() - 1 {
            if let None = self.current_token {
                self.token_index = 0;
            } else {
                self.token_index += 1;
            }
            let token = &self.tokens[self.token_index];
            if let TokenType::Space | TokenType::Newline = token.token_type {
                continue;
            }
            self.current_token = Some(self.tokens[self.token_index].clone());
            return;
        }
        self.current_token = None;
    }
}
