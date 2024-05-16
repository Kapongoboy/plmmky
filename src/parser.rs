use crate::ast;
use crate::lexer::Lexer;
use crate::token::{Token, TokenKind};

pub struct Parser<'a> {
    lex: &'a mut Lexer<'a>,
    cur_token: Token<'a>,
    peek_token: Token<'a>,
}

impl<'a> Parser<'a> {
    pub fn new(lex: &'a mut Lexer<'a>) -> Parser {
        let mut p = Parser {
            lex,
            cur_token: Token::new(TokenKind::EOF, None),
            peek_token: Token::new(TokenKind::EOF, None),
        };
        p.next_token();
        p.next_token();
        p
    }

    fn next_token(&mut self) {
        self.cur_token = std::mem::take(&mut self.peek_token);
        self.peek_token = self.lex.next_token();
    }

    pub fn parse_program(&self) -> Option<ast::Program> {
        None
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    fn test_let_statement(s: &Box<dyn ast::Statement>, test_name: &str) {
        assert_eq!(s.token_literal(), test_name);
    }

    #[test]
    fn test_let_statements() {
        let input = "let x = 5;\n\
        let y = 10;\n\
        let foobar = 838383;";

        let mut l = Lexer::new(input, true, None);
        let p = Parser::new(&mut l);

        let program = p.parse_program();

        assert_ne!(program, None);

        assert_eq!(&program.as_ref().expect("Program should be Some here").statements.len(), &3);

        let expected_identifiers = vec!["x", "y", "foobar"];

        for (stmt, idtfr) in program.expect("Program should be Some here").statements.iter().zip(expected_identifiers.iter()) {
            test_let_statement(stmt, idtfr);
        }
    }
}
