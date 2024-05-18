use crate::token::Token;
use std::fmt::{Debug, Formatter, Result};

pub trait Node {
    fn token_literal(&self) -> String;
}

pub trait Expression: Node {
    fn expression_node(&self);
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

pub struct LetInternal<'a> {
    token: Token<'a>,
    name: Option<Box<Identifier<'a>>>,
    value: Option<Box<dyn Expression>>,
}

impl<'a> LetInternal<'a> {
    pub fn new(
        token: Token<'a>,
        name: Option<Identifier<'a>>,
        value: Option<Box<dyn Expression>>,
    ) -> LetInternal<'a> {
        let result = match name {
            Some(n) => Some(Box::new(n)),
            None => None,
        };
        LetInternal {
            token,
            name: result,
            value,
        }
    }

    pub fn change_name(&mut self, name: Identifier<'a>) {
        self.name = Some(Box::new(name));
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

pub enum Statement<'a> {
    Let(LetInternal<'a>),
    Return,
}

pub struct Program<'a> {
    pub statements: Vec<Statement<'a>>,
}

impl Debug for Program<'_> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let mut s = String::new();
        for stmt in &self.statements {
            match stmt {
                Statement::Let(i) => {
                    s.push_str("let ");
                    s.push_str(&i.name().unwrap().value);
                    s.push_str(" = ");
                }
                _ => (),
            }
        }
        write!(f, "{}", s)
    }
}

impl PartialEq for Program<'_> {
    fn eq(&self, other: &Self) -> bool {
        for (left, right) in self.statements.iter().zip(other.statements.iter()) {
            match (left, right) {
                (Statement::Let(l), Statement::Let(r)) => {
                    if l.name().unwrap().value != r.name().unwrap().value {
                        return false;
                    }
                }
                (_, _) => (),
            }
        }
        true
    }
}