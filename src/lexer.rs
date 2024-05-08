use crate::token::{TokenKind, Token};

pub struct Lexer {
    input: String,
    position: usize, // current position
    read_position: usize, // next position
    ch: char,
}

impl Lexer {
    pub fn new(s: &str) -> Lexer {
        let mut l = Lexer {
            input: s.to_string(),
            position: 0,
            read_position: 0,
            ch: ' ',
        };
        l.read_char();
        l
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

    pub fn next_token(&mut self) -> Token {
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
            _ => Token::new(TokenKind::ILLEGAL, None),
        };
        self.read_char();
        tok
    }
}

#[cfg(test)]
pub mod tests {
    use crate::token::{Token,TokenKind};
    use crate::lexer::Lexer;

    #[test]
    fn test_next_token(){
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
}
