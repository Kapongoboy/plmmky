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

    fn cur_token_is(&self, t: TokenKind) -> bool {
        self.cur_token.ttype == t
    }

    fn peek_token_is(&self, t: TokenKind) -> bool {
        self.peek_token.ttype == t
    }

    fn expect_peek(&mut self, t: TokenKind) -> bool {
        match self.peek_token_is(t) {
            true => {
                self.next_token();
                true
            }
            false => false,
        }
    }

    fn parse_let_statement(&mut self) -> Option<ast::LetStatement<'a>> {
        let old_cur_token = self.cur_token.clone();

        if !self.expect_peek(TokenKind::IDENT(String::from("something"))) {
            return None;
        }

        let id = ast::Identifier::new(self.cur_token.clone(), self.cur_token.literal.clone());
        let stmt = ast::LetStatement::new(old_cur_token, Some(id), None);

        if !self.expect_peek(TokenKind::ASSIGN) {
            return None;
        }

        Some(stmt)
    }

    fn parse_statement(&mut self) -> Option<Box<dyn ast::Statement + 'a>> {
        match self.cur_token.ttype {
            TokenKind::LET => {
                let stmt = self.parse_let_statement();
                match stmt {
                    Some(i) => Some(Box::new(i)),
                    None => panic!("implementation error, could make let statement"),
                }
            }
            _ => None,
        }
    }

    pub fn parse_program(&mut self) -> Option<ast::Program> {
        let mut program = ast::Program { statements: vec![] };

        while self.cur_token.ttype != TokenKind::EOF {
            let stmt = self.parse_statement();

            match stmt {
                Some(s) => program.statements.push(s),
                None => (),
            }

            self.next_token();
        }

        Some(program)
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
        let mut p = Parser::new(&mut l);

        let program = p.parse_program();

        assert_ne!(program, None);

        assert_eq!(
            &program
                .as_ref()
                .expect("Program should be Some here")
                .statements
                .len(),
            &3
        );

        let expected_identifiers = vec!["x", "y", "foobar"];

        for (stmt, idtfr) in program
            .expect("Program should be Some here")
            .statements
            .iter()
            .zip(expected_identifiers.iter())
        {
            test_let_statement(stmt, idtfr);
        }
    }
}
