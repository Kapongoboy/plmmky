#[cfg(test)]
pub mod tests {
    use crate::token::{Token,TokenType};

    #[test]
    fn test_next_token(){
        let input = "=+(){},;";
        let test_arr = [
            Token::new(TokenType::ASSIGN, None),
            Token::new(TokenType::PLUS, None),
            Token::new(TokenType::LPAREN, None),
            Token::new(TokenType::RPAREN, None),
            Token::new(TokenType::LBRACE, None),
            Token::new(TokenType::RBRACE, None),
            Token::new(TokenType::COMMA, None),
            Token::new(TokenType::SEMICOLON, None),
            Token::new(TokenType::EOF, None),
        ];

        let l = Lexer::new(input);

        for (i, tt) in test_arr.iter().enumerate() {
            let tok = l.next_token();
            assert_eq!(tok.ttype, tt.ttype);
            assert_eq!(tok.literal, tt.literal)
        }
    }
}
