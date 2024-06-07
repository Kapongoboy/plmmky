use crate::ast;
use crate::lexer::Lexer;
use crate::token::{Token, TokenKind};

pub struct Parser<'a> {
    lex: Lexer<'a>,
    cur_token: Token<'a>,
    peek_token: Token<'a>,
    errors: Vec<String>,
}

impl<'a> Parser<'a> {
    pub fn new(lex: Lexer<'a>) -> Parser {
        let mut p = Parser {
            lex,
            cur_token: Token::new(TokenKind::EOF, None),
            peek_token: Token::new(TokenKind::EOF, None),
            errors: Vec::new(),
        };
        p.next_token();
        p.next_token();
        p
    }

    pub fn errors(&self) -> &[String] {
        &self.errors
    }

    fn peek_error(&mut self, t: &TokenKind) {
        let msg = format!("expected next token to be {}, got {} instead", t, self.peek_token.ttype);
        self.errors.push(msg);
    }

    fn next_token(&mut self) {
        self.cur_token = std::mem::take(&mut self.peek_token);
        self.peek_token = self.lex.next_token();
    }

    fn cur_token_is(&self, t: &TokenKind) -> bool {
        self.cur_token.ttype == *t
    }

    fn peek_token_is(&self, t: &TokenKind) -> bool {
        match t {
            TokenKind::IDENT(_) => matches!(self.peek_token.ttype, TokenKind::IDENT(_)),
            _ => self.peek_token.ttype == *t,
        }
    }

    fn expect_peek(&mut self, t: &TokenKind) -> bool {
        if self.peek_token_is(t) {
                self.next_token();
                true
        } else {
            self.peek_error(t);
            false
        }
    }

    fn parse_let_statement(&mut self) -> Option<ast::Statement<'a>> {
        let mut internal = ast::LetInternal::new(self.cur_token.clone(), None, None);

        if !self.expect_peek(&TokenKind::IDENT(String::from("something"))) {
            return None;
        }

        internal.change_name(ast::Identifier::new(
            self.cur_token.clone(),
            self.cur_token.literal.clone(),
        ));

        if !self.expect_peek(&TokenKind::ASSIGN) {
            return None;
        }

        while !self.cur_token_is(&TokenKind::SEMICOLON) {
            self.next_token();
        }

        Some(ast::Statement::Let(internal))
    }

    fn parse_statement<'b>(&mut self) -> Option<ast::Statement<'b>>
    where
        'a: 'b,
    {
        match self.cur_token.ttype {
            TokenKind::LET => {
                let stmt = self.parse_let_statement();
                match stmt {
                    Some(i) => Some(i),
                    None => panic!("implementation error, could not make let statement"),
                }
            }
            _ => None,
        }
    }

    pub fn parse_program(&mut self) -> Option<ast::Program<'a>> {
        let mut program = ast::Program { statements: vec![] };

        while !self.cur_token_is(&TokenKind::EOF) {
            let stmt = self.parse_statement();

            if let Some(s) = stmt {
                program.statements.push(s);
            }

            self.next_token();
        }

        Some(program)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_let_statements() {
        let input = "let x = 5;\n\
        let y = 10;\n\
        let foobar = 838383;";

        let l = Lexer::new(input, true, None);
        let mut p = Parser::new(l);

        let program = p.parse_program();

        assert_eq!(p.errors().len(), 0);

        assert_ne!(program, None);

        assert_eq!(
            &program
                .as_ref()
                .expect("Program should be Some here")
                .statements
                .len(),
            &3
        );

        let expected_identifiers: [&str; 3] = ["x", "y", "foobar"];

        let check_let_statement = |stmt: &ast::Statement, expected_name: &str| match stmt {
            ast::Statement::Let(i) => {
                assert_eq!(
                    i.name().expect("couldn't get the name").value(),
                    expected_name
                )
            }
            _ => panic!("expected let statement but got something else"),
        };

        for (stmt, name) in program
            .expect("Program should be Some here")
            .statements
            .iter()
            .zip(expected_identifiers.iter())
        {
            check_let_statement(stmt, name)
        }
    }
}
