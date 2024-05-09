use crate::token;
use crate::token::{Token, TokenKind};
use std::path::Path;

pub struct Lexer<'a> {
    input: String,
    position: usize,      // current position
    read_position: usize, // next position
    ch: char,
    curr_line: usize,
    curr_col: usize,
    repl: bool,
    path: Option<&'a Path>,
}

impl<'a> Lexer<'a> {
    pub fn new(s: &str, repl: bool, path: Option<&'a Path>) -> Lexer<'a> {
        let mut l = Lexer {
            input: s.trim().to_string(),
            position: 0,
            read_position: 0,
            ch: ' ',
            curr_line: 1,
            curr_col: 1,
            repl,
            path,
        };
        l.read_char();
        l
    }

    fn skip_whitespace(&mut self) {
        while self.ch == ' ' || self.ch == '\n' || self.ch == '\t' || self.ch == '\r' {
            self.read_char();
        }
    }

    fn peek_char(&self) -> char {
        if self.read_position >= self.input.len() {
            '\0'
        } else {
            self.input.chars().collect::<Vec<char>>()[self.read_position]
        }
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = '\0';
        } else {
            self.ch = self.input.chars().collect::<Vec<char>>()[self.read_position];
        }

        if self.ch == '\n' {
            self.curr_line += 1;
            self.curr_col = 1;
        } else {
            self.curr_col += 1;
        }

        self.position = self.read_position;
        self.read_position += 1;
    }

    fn read_number(&mut self) -> Token {
        let position = self.position;

        while self.ch.is_digit(10) {
            self.read_char();
        }

        let loc = if self.repl {
            None
        } else {
            Some(token::Location::new(
                self.curr_line,
                self.curr_col,
                self.path.expect("There is a problem with the file"),
            ))
        };

        Token::new(
            TokenKind::INT(
                self.input.chars().collect::<Vec<char>>()[position..self.position]
                    .iter()
                    .collect::<String>()
                    .parse::<i128>()
                    .unwrap(),
            ),
            None,
        )
    }

    fn read_identifier(&mut self) -> Token {
        let position = self.position;

        while self.ch.is_alphabetic() || self.ch == '_' {
            self.read_char();
        }

        let loc = if self.repl {
            None
        } else {
            Some(token::Location::new(
                self.curr_line,
                self.curr_col,
                self.path.expect("There is a problem with the file"),
            ))
        };

        Token::new(
            token::lookup_ident(
                self.input.chars().collect::<Vec<char>>()[position..self.position]
                    .iter()
                    .collect::<String>()
                    .as_str(),
            ),
            None,
        )
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        let loc = if self.repl {
            None
        } else {
            Some(token::Location::new(
                self.curr_line,
                self.curr_col,
                self.path.expect("There is a problem with the file"),
            ))
        };

        let tok = match self.ch {
            '=' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    Token::new(TokenKind::EQ, loc)
                } else {
                    Token::new(TokenKind::ASSIGN, loc)
                }
            },
            ';' => Token::new(TokenKind::SEMICOLON, loc),
            '(' => Token::new(TokenKind::LPAREN, loc),
            ')' => Token::new(TokenKind::RPAREN, loc),
            ',' => Token::new(TokenKind::COMMA, loc),
            '+' => Token::new(TokenKind::PLUS, loc),
            '{' => Token::new(TokenKind::LBRACE, loc),
            '}' => Token::new(TokenKind::RBRACE, loc),
            '!' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    Token::new(TokenKind::NEQ, loc)
                } else {
                    Token::new(TokenKind::BANG, loc)
                }
            },
            '-' => Token::new(TokenKind::MINUS, loc),
            '*' => Token::new(TokenKind::ASTERISK, loc),
            '/' => Token::new(TokenKind::SLASH, loc),
            '<' => Token::new(TokenKind::LT, loc),
            '>' => Token::new(TokenKind::GT, loc),
            '\0' => Token::new(TokenKind::EOF, loc),
            _ => {
                if self.ch.is_alphabetic() || self.ch == '_' {
                    return self.read_identifier();
                } else if self.ch.is_digit(10) {
                    return self.read_number();
                } else {
                    Token::new(TokenKind::ILLEGAL, loc)
                }
            }
        };
        self.read_char();
        tok
    }
}

#[cfg(test)]
pub mod tests {
    use crate::lexer::Lexer;
    use crate::token::{Location, Token, TokenKind};

    #[test]
    fn test_next_token_file() {
        let path = std::path::Path::new("assets/token_file_test.my");
        let file = std::fs::read_to_string(path);

        match file {
            Ok(s) => {
                let input = &s;
                let test_arr = [
                    Token::new(TokenKind::LET, Some(Location::new(1, 1, path))),
                    Token::new(
                        TokenKind::IDENT(String::from("five")),
                        Some(Location::new(1, 5, path)),
                    ),
                    Token::new(TokenKind::ASSIGN, Some(Location::new(1, 10, path))),
                    Token::new(TokenKind::INT(5), Some(Location::new(1, 12, path))),
                    Token::new(TokenKind::SEMICOLON, Some(Location::new(1, 13, path))),
                    Token::new(TokenKind::LET, Some(Location::new(2, 1, path))),
                    Token::new(
                        TokenKind::IDENT(String::from("ten")),
                        Some(Location::new(2, 5, path)),
                    ),
                    Token::new(TokenKind::ASSIGN, Some(Location::new(2, 10, path))),
                    Token::new(TokenKind::INT(10), Some(Location::new(2, 12, path))),
                    Token::new(TokenKind::SEMICOLON, Some(Location::new(2, 14, path))),
                    Token::new(TokenKind::LET, Some(Location::new(3, 1, path))),
                    Token::new(
                        TokenKind::IDENT(String::from("add")),
                        Some(Location::new(3, 5, path)),
                    ),
                    Token::new(TokenKind::ASSIGN, Some(Location::new(3, 10, path))),
                    Token::new(TokenKind::FUNCTION, Some(Location::new(3, 12, path))),
                    Token::new(TokenKind::LPAREN, Some(Location::new(3, 20, path))),
                    Token::new(
                        TokenKind::IDENT(String::from("x")),
                        Some(Location::new(3, 21, path)),
                    ),
                    Token::new(TokenKind::COMMA, Some(Location::new(3, 22, path))),
                    Token::new(
                        TokenKind::IDENT(String::from("y")),
                        Some(Location::new(3, 24, path)),
                    ),
                    Token::new(TokenKind::RPAREN, Some(Location::new(3, 25, path))),
                    Token::new(TokenKind::LBRACE, Some(Location::new(3, 27, path))),
                    Token::new(
                        TokenKind::IDENT(String::from("x")),
                        Some(Location::new(4, 5, path)),
                    ),
                    Token::new(TokenKind::PLUS, Some(Location::new(4, 7, path))),
                    Token::new(
                        TokenKind::IDENT(String::from("y")),
                        Some(Location::new(4, 9, path)),
                    ),
                    Token::new(TokenKind::SEMICOLON, Some(Location::new(4, 10, path))),
                    Token::new(TokenKind::RBRACE, Some(Location::new(5, 1, path))),
                    Token::new(TokenKind::SEMICOLON, Some(Location::new(5, 2, path))),
                    Token::new(TokenKind::LET, Some(Location::new(7, 1, path))),
                    Token::new(
                        TokenKind::IDENT(String::from("result")),
                        Some(Location::new(7, 5, path)),
                    ),
                    Token::new(TokenKind::ASSIGN, Some(Location::new(7, 12, path))),
                    Token::new(
                        TokenKind::IDENT(String::from("add")),
                        Some(Location::new(7, 14, path)),
                    ),
                    Token::new(TokenKind::LPAREN, Some(Location::new(7, 17, path))),
                    Token::new(
                        TokenKind::IDENT(String::from("five")),
                        Some(Location::new(7, 18, path)),
                    ),
                    Token::new(TokenKind::COMMA, Some(Location::new(7, 23, path))),
                    Token::new(
                        TokenKind::IDENT(String::from("ten")),
                        Some(Location::new(7, 25, path)),
                    ),
                    Token::new(TokenKind::RPAREN, Some(Location::new(7, 28, path))),
                    Token::new(TokenKind::SEMICOLON, Some(Location::new(7, 29, path))),
                    Token::new(TokenKind::EOF, None),
                ];

                let mut l = Lexer::new(input, false, Some(path));

                for tt in test_arr.iter() {
                    let tok = l.next_token();
                    assert_eq!(tok.ttype, tt.ttype);
                    assert_eq!(tok.literal, tt.literal)
                }
            }
            Err(e) => {
                println!("Error reading file: {}", e);
                assert!(false);
            }
        }
    }
    #[test]
    fn next_token_full_set() {
        let input = "let five = 5;\n\
            let ten = 10;\n\
            let add = fn(x, y) {\n\
            x + y;\n\
            };\n\
            let result = add(five, ten);\n\
            !-/*5;\n\
            5 < 10 > 5;\n\
            \n\
            if (5 < 10) {\n\
                return true;\n\
            } else {\n\
                return false;\n\
            }\n\
            \n\
            10 == 10;\n\
            10 != 9;\n\
            ";

        let test_arr = [
            Token::new(TokenKind::LET, None),
            Token::new(TokenKind::IDENT(String::from("five")), None),
            Token::new(TokenKind::ASSIGN, None),
            Token::new(TokenKind::INT(5), None),
            Token::new(TokenKind::SEMICOLON, None),
            Token::new(TokenKind::LET, None),
            Token::new(TokenKind::IDENT(String::from("ten")), None),
            Token::new(TokenKind::ASSIGN, None),
            Token::new(TokenKind::INT(10), None),
            Token::new(TokenKind::SEMICOLON, None),
            Token::new(TokenKind::LET, None),
            Token::new(TokenKind::IDENT(String::from("add")), None),
            Token::new(TokenKind::ASSIGN, None),
            Token::new(TokenKind::FUNCTION, None),
            Token::new(TokenKind::LPAREN, None),
            Token::new(TokenKind::IDENT(String::from("x")), None),
            Token::new(TokenKind::COMMA, None),
            Token::new(TokenKind::IDENT(String::from("y")), None),
            Token::new(TokenKind::RPAREN, None),
            Token::new(TokenKind::LBRACE, None),
            Token::new(TokenKind::IDENT(String::from("x")), None),
            Token::new(TokenKind::PLUS, None),
            Token::new(TokenKind::IDENT(String::from("y")), None),
            Token::new(TokenKind::SEMICOLON, None),
            Token::new(TokenKind::RBRACE, None),
            Token::new(TokenKind::SEMICOLON, None),
            Token::new(TokenKind::LET, None),
            Token::new(TokenKind::IDENT(String::from("result")), None),
            Token::new(TokenKind::ASSIGN, None),
            Token::new(TokenKind::IDENT(String::from("add")), None),
            Token::new(TokenKind::LPAREN, None),
            Token::new(TokenKind::IDENT(String::from("five")), None),
            Token::new(TokenKind::COMMA, None),
            Token::new(TokenKind::IDENT(String::from("ten")), None),
            Token::new(TokenKind::RPAREN, None),
            Token::new(TokenKind::SEMICOLON, None),
            Token::new(TokenKind::BANG, None),
            Token::new(TokenKind::MINUS, None),
            Token::new(TokenKind::SLASH, None),
            Token::new(TokenKind::ASTERISK, None),
            Token::new(TokenKind::INT(5), None),
            Token::new(TokenKind::SEMICOLON, None),
            Token::new(TokenKind::INT(5), None),
            Token::new(TokenKind::LT, None),
            Token::new(TokenKind::INT(10), None),
            Token::new(TokenKind::GT, None),
            Token::new(TokenKind::INT(5), None),
            Token::new(TokenKind::SEMICOLON, None),
            Token::new(TokenKind::IF, None),
            Token::new(TokenKind::LPAREN, None),
            Token::new(TokenKind::INT(5), None),
            Token::new(TokenKind::LT, None),
            Token::new(TokenKind::INT(10), None),
            Token::new(TokenKind::RPAREN, None),
            Token::new(TokenKind::LBRACE, None),
            Token::new(TokenKind::RETURN, None),
            Token::new(TokenKind::TRUE, None),
            Token::new(TokenKind::SEMICOLON, None),
            Token::new(TokenKind::RBRACE, None),
            Token::new(TokenKind::ELSE, None),
            Token::new(TokenKind::LBRACE, None),
            Token::new(TokenKind::RETURN, None),
            Token::new(TokenKind::FALSE, None),
            Token::new(TokenKind::SEMICOLON, None),
            Token::new(TokenKind::RBRACE, None),
            Token::new(TokenKind::INT(10), None),
            Token::new(TokenKind::EQ, None),
            Token::new(TokenKind::INT(10), None),
            Token::new(TokenKind::SEMICOLON, None),
            Token::new(TokenKind::INT(10), None),
            Token::new(TokenKind::NEQ, None),
            Token::new(TokenKind::INT(9), None),
            Token::new(TokenKind::SEMICOLON, None),
            Token::new(TokenKind::EOF, None),
        ];

        let mut l = Lexer::new(input, true, None);

        for tt in test_arr.iter() {
            let tok = l.next_token();
            assert_eq!(tok.ttype, tt.ttype);
            assert_eq!(tok.literal, tt.literal)
        }
    }
}
