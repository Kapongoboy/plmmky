use crate::token;
use crate::token::{Token, TokenKind};

pub struct Lexer {
    input: String,
    position: usize,      // current position
    read_position: usize, // next position
    ch: char,
}

impl Lexer {
    pub fn new(s: &str) -> Lexer {
        let mut l = Lexer {
            input: s.trim().to_string(),
            position: 0,
            read_position: 0,
            ch: ' ',
        };
        l.read_char();
        l
    }

    fn skip_whitespace(&mut self) {
        while self.ch == ' ' || self.ch == '\n' || self.ch == '\t' || self.ch == '\r' {
            self.read_char();
        }
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = '\0';
        } else {
            self.ch = self.input.chars().collect::<Vec<char>>()[self.read_position];
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    fn read_number(&mut self) -> Token {
        let position = self.position;

        while self.ch.is_digit(10) {
            self.read_char();
        }

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

        let tok = match self.ch {
            '=' => Token::new(TokenKind::ASSIGN, None),
            ';' => Token::new(TokenKind::SEMICOLON, None),
            '(' => Token::new(TokenKind::LPAREN, None),
            ')' => Token::new(TokenKind::RPAREN, None),
            ',' => Token::new(TokenKind::COMMA, None),
            '+' => Token::new(TokenKind::PLUS, None),
            '{' => Token::new(TokenKind::LBRACE, None),
            '}' => Token::new(TokenKind::RBRACE, None),
            '\0' => Token::new(TokenKind::EOF, None),
            _ => {
                if self.ch.is_alphabetic() || self.ch == '_' {
                    return self.read_identifier();
                } else if self.ch.is_digit(10) {
                    return self.read_number();
                } else {
                    Token::new(TokenKind::ILLEGAL, None)
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
    use crate::token::{Token, TokenKind};

    #[test]
    fn test_next_token() {
        let input = "=+(){},;";
        let test_arr = [
            Token::new(TokenKind::ASSIGN, None),
            Token::new(TokenKind::PLUS, None),
            Token::new(TokenKind::LPAREN, None),
            Token::new(TokenKind::RPAREN, None),
            Token::new(TokenKind::LBRACE, None),
            Token::new(TokenKind::RBRACE, None),
            Token::new(TokenKind::COMMA, None),
            Token::new(TokenKind::SEMICOLON, None),
            Token::new(TokenKind::EOF, None),
        ];

        let mut l = Lexer::new(input);

        for tt in test_arr.iter() {
            let tok = l.next_token();
            assert_eq!(tok.ttype, tt.ttype);
            assert_eq!(tok.literal, tt.literal)
        }
    }

    #[test]
    fn test_next_token_expanded() {
        let input = "let five = 5;\n\
        let ten = 10;\n\
        let add = fn(x, y) {\n\
            x + y;\n\
        };\n\
        \
        let result = add(five, ten);";

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
            Token::new(TokenKind::EOF, None),
        ];

        let mut l = Lexer::new(input);

        for tt in test_arr.iter() {
            let tok = l.next_token();
            println!("tok lit = {}, tt lit = {}", tok.literal, tt.literal);
            assert_eq!(tok.ttype, tt.ttype);
            assert_eq!(tok.literal, tt.literal)
        }
    }
}
