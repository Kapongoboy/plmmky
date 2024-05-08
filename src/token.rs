use std::path::Path;

#[derive(Debug, PartialEq)]
pub enum TokenKind {
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
    pub ttype: TokenKind,
    pub literal: String,
    local: Option<Location<'a>>,
}

impl<'a> Token<'a> {
    pub fn new(token_type: TokenKind, local: Option<Location>) -> Token {
        let literal = match token_type {
            TokenKind::EOF => String::from(""),
            TokenKind::ILLEGAL => String::from("ILLEGAL"),
            TokenKind::IDENT(ref s) => s.clone(),
            TokenKind::INT(i) => i.to_string(),
            TokenKind::ASSIGN => String::from("="),
            TokenKind::PLUS => String::from("+"),
            TokenKind::COMMA => String::from(","),
            TokenKind::SEMICOLON => String::from(";"),
            TokenKind::LPAREN => String::from("("),
            TokenKind::RPAREN => String::from(")"),
            TokenKind::LBRACE => String::from("{"),
            TokenKind::RBRACE => String::from("}"),
            TokenKind::FUNCTION => String::from("FUNCTION"),
            TokenKind::LET => String::from("LET"),
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
