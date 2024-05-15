use crate::ast;
use crate::lexer::Lexer;
use crate::token::{Token, TokenKind};

struct Parser<'a> {
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
