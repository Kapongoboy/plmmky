use phf::phf_map;
use std::borrow::Borrow;
use std::path::Path;
use std::fmt::Display;
use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub enum TokenKind {
    ILLEGAL,
    EOF,

    // Identifiers + literals
    IDENT(String),
    INT(i128),

    // Operators
    ASSIGN,
    PLUS,
    MINUS,
    BANG,
    ASTERISK,
    SLASH,

    LT,
    GT,

    EQ,
    NEQ,

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
    TRUE,
    FALSE,
    IF,
    ELSE,
    RETURN,
}

impl Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg = match self {
            TokenKind::ILLEGAL => "ILLEGAL".to_string(),
            TokenKind::EOF => "EOF".to_string(),

            // Identifiers + literals
            TokenKind::IDENT(inner_string) => "IDENT = ".to_owned() + inner_string,
            TokenKind::INT(inner_int) => "INT = ".to_owned() + inner_int.to_string().borrow(),

            // Operators
            TokenKind::ASSIGN => "ASSIGN".to_string(),
            TokenKind::PLUS => "PLUS".to_string(),
            TokenKind::MINUS => "MINUS".to_string(),
            TokenKind::BANG => "BANG".to_string(),
            TokenKind::ASTERISK => "ASTERISK".to_string(),
            TokenKind::SLASH => "SLASH".to_string(),

            TokenKind::LT => "LT".to_string(),
            TokenKind::GT => "GT".to_string(),

            TokenKind::EQ => "EQ".to_string(),
            TokenKind::NEQ => "NEQ".to_string(),

            // Delimiters
            TokenKind::COMMA => "COMMA".to_string(),
            TokenKind::SEMICOLON => "SEMICOLON".to_string(),

            TokenKind::LPAREN => "LPAREN".to_string(),
            TokenKind::RPAREN => "RPAREN".to_string(),
            TokenKind::LBRACE => "LBRACE".to_string(),
            TokenKind::RBRACE => "RBRACE".to_string(),

            // Keywords
            TokenKind::FUNCTION => "FUNCTION".to_string(),
            TokenKind::LET => "LET".to_string(),
            TokenKind::TRUE => "TRUE".to_string(),
            TokenKind::FALSE => "FALSE".to_string(),
            TokenKind::IF => "IF".to_string(),
            TokenKind::ELSE => "ELSE".to_string(),
            TokenKind::RETURN => "RETURN".to_string(),
        };

        write!(f, "{}", msg)
    }
}

#[derive(Debug, Clone)]
pub struct Location<'a> {
    row: usize,
    col: usize,
    file: &'a Path,
}

impl<'a> Location<'a> {
    pub fn new(row: usize, col: usize, file: &Path) -> Location {
        Location { row, col, file }
    }

    pub fn get(&self) -> (& usize, & usize, &'a Path) {
        (&self.row, &self.col, self.file)
    }
}

#[derive(Debug, Clone)]
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
            TokenKind::MINUS => String::from("-"),
            TokenKind::BANG => String::from("!"),
            TokenKind::ASTERISK => String::from("*"),
            TokenKind::SLASH => String::from("/"),
            TokenKind::LT => String::from("<"),
            TokenKind::GT => String::from(">"),
            TokenKind::EQ => String::from("=="),
            TokenKind::NEQ => String::from("!="),
            TokenKind::COMMA => String::from(","),
            TokenKind::SEMICOLON => String::from(";"),
            TokenKind::LPAREN => String::from("("),
            TokenKind::RPAREN => String::from(")"),
            TokenKind::LBRACE => String::from("{"),
            TokenKind::RBRACE => String::from("}"),
            TokenKind::FUNCTION => String::from("FUNCTION"),
            TokenKind::LET => String::from("LET"),
            TokenKind::TRUE => String::from("TRUE"),
            TokenKind::FALSE => String::from("FALSE"),
            TokenKind::IF => String::from("IF"),
            TokenKind::ELSE => String::from("ELSE"),
            TokenKind::RETURN => String::from("RETURN"),
        };

        Token {
            ttype: token_type,
            literal,
            local,
        }
    }

    pub fn local(&self) -> Option<& Location<'a>> {
        match &self.local {
            Some(i) => Some(i),
            None => None
                
        }
    }
}

impl<'a> Default for Token<'a> {
    fn default() -> Token<'a> {
        Token {
            ttype: TokenKind::EOF,
            literal: String::from(""),
            local: None,
        }
    }
}

static KEYWORDS: phf::Map<&'static str, TokenKind> = phf_map! {
    "let" => TokenKind::LET,
    "fn" => TokenKind::FUNCTION,
    "true" => TokenKind::TRUE,
    "false" => TokenKind::FALSE,
    "if" => TokenKind::IF,
    "else" => TokenKind::ELSE,
    "return" => TokenKind::RETURN,
};

pub fn lookup_ident(ident: &str) -> TokenKind {
    match KEYWORDS.get(ident) {
        Some(t) => t.clone(),
        None => TokenKind::IDENT(ident.to_string()),
    }
}
