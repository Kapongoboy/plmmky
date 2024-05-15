use crate::token::Token;

trait Node {
    fn token_literal(&self) -> String;
}

trait Statement: Node {
    fn statement_node(&self);
}

trait Expression: Node {
    fn expression_node(&self);
}

pub struct Program {
    statements: Vec<Box<dyn Statement>>,
}

impl Program {
    pub fn token_literal(&self) -> String {
        match self.statements.len() {
            0 => String::from(""),
            _ => self.statements[0].token_literal(),
        }
    }
}

struct Identifier<'a> {
    token: Token<'a>,
    // value: String, This is probably not needed seeing as we have a string inside our Ident enum
}

struct LetStatement<'a> {
    token: Token<'a>,
    name: &'a Identifier<'a>,
    value: Box<dyn Expression>,
}

impl<'a> Node for LetStatement<'a> {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
}

impl<'a> Statement for LetStatement<'a> {
    fn statement_node(&self) {}
}

impl<'a> Node for Identifier<'a> {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
}

impl<'a> Expression for Identifier<'a> {
    fn expression_node(&self) {}
}
