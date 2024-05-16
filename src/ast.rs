use crate::token::Token;
use std::cmp::PartialEq;
use std::fmt::{Debug, Formatter, Result};

pub trait Node {
    fn token_literal(&self) -> String;
}

pub trait Statement: Node {
    fn statement_node(&self);
}

pub trait Expression: Node {
    fn expression_node(&self);
}

pub struct Program {
    pub statements: Vec<Box<dyn Statement>>,
}

impl Debug for Program {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let mut s = String::new();
        for stmt in &self.statements {
            s.push_str(&stmt.token_literal());
        }
        write!(f, "{}", s)
    }
}

impl PartialEq for Program {
    fn eq(&self, other: &Self) -> bool {
        for (left, right) in self.statements.iter().zip(other.statements.iter()) {
            if left.token_literal() != right.token_literal() {
                return false;
            }
        }
        true
    }
}

impl Program {
    pub fn token_literal(&self) -> String {
        match self.statements.len() {
            0 => String::from(""),
            _ => self.statements[0].token_literal(),
        }
    }
}

pub struct Identifier<'a> {
    token: Token<'a>,
    value: String,
}

impl Identifier<'_> {
    pub fn new(token: Token, value: String) -> Identifier {
        Identifier { token, value }
    }

    pub fn token(&self) -> &Token {
        &self.token
    }

    pub fn value(&self) -> &String {
        &self.value
    }
}

pub struct LetStatement<'a> {
    token: Token<'a>,
    name: Option<Box<Identifier<'a>>>,
    value: Option<Box<dyn Expression>>,
}

impl<'a> LetStatement<'a> {
    pub fn new(
        token: Token<'a>,
        name: Option<Identifier<'a>>,
        value: Option<Box<dyn Expression>>,
    ) -> LetStatement<'a> {
        let result = match name {
            Some(n) => Some(Box::new(n)),
            None => None,
        };
        LetStatement {
            token,
            name: result,
            value,
        }
    }

    pub fn token(&self) -> &Token<'a> {
        &self.token
    }

    pub fn name(&self) -> Option<&Box<Identifier<'a>>> {
        self.name.as_ref()
    }

    pub fn value(&self) -> Option<&Box<dyn Expression>> {
        self.value.as_ref()
    }
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