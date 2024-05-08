use std::path::Path;

pub enum TokenType {
    ILLEGAL,
    EOF,

// Identifiers + literals
    IDENT(String),
    INT(usize),

// Operators
    ASSIGN,
    PLUS,

// Delimiters
    COMMA,
    SEMICOLON,


    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,

// Keywords
    FUNCTION,
    LET,
}

pub struct Location<'a> {
    row: usize,
    col: usize,
    file: &'a Path,
}

impl <'a> Location<'a> {
    pub fn new(row: usize, col: usize, path: &str) -> Location {
        Location {
            row,
            col,
            file: Path::new(path)
        }
    }
}

pub struct Token<'a> {
    pub ttype: TokenType,
    pub literal: String,
    local: Option<Location<'a>>,
}

impl<'a> Token<'a> {
    pub fn new(token_type: TokenType, local: Option<Location>) -> Token {
        let literal = match token_type {
            TokenType::EOF => String::from(""),
            TokenType::ILLEGAL => String::from("ILLEGAL"),
            TokenType::IDENT(ref s) => s.clone(),
            TokenType::INT(i) => i.to_string(),
            TokenType::ASSIGN => String::from("="),
            TokenType::PLUS => String::from("+"),
            TokenType::COMMA => String::from(","),
            TokenType::SEMICOLON => String::from(";"),
            TokenType::LPAREN => String::from("("),
            TokenType::RPAREN => String::from(")"),
            TokenType::LBRACE => String::from("{"),
            TokenType::RBRACE => String::from("}"),
            TokenType::FUNCTION => String::from("FUNCTION"),
            TokenType::LET => String::from("LET"),
        };

        Token {
            ttype: token_type,
            literal,
            local: match local {
                Some(l) => Some(l),
                None => None
            }
        }
    }
}
