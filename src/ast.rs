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
    name: Option<Identifier<'a>>,
    value: Option<Box<dyn Expression>>,
}

impl<'a> LetInternal<'a> {
    pub fn new(
        token: Token<'a>,
        name: Option<Identifier<'a>>,
        value: Option<Box<dyn Expression>>,
    ) -> LetInternal<'a> {
        LetInternal { token, name, value }
    }

    pub fn change_name(&mut self, name: Identifier<'a>) {
        self.name = Some(name);
    }

    pub fn token(&self) -> &Token<'a> {
        &self.token
    }

    pub fn name(&self) -> Option<&Identifier<'a>> {
        self.name.as_ref()
    }

    pub fn value(&self) -> Option<&Box<dyn Expression>> {
        self.value.as_ref()
    }
}

pub struct ReturnInternal<'a> {
    token: Token<'a>,
    return_value: Option<Box<dyn Expression>>,
}

impl<'a> ReturnInternal<'a> {
    pub fn init(token: Token<'a>, return_value: Option<Box<dyn Expression>>) -> Self {
        Self {
            token,
            return_value,
        }
    }

    pub fn token_literal(&self) -> &str {
        &self.token.literal
    }
}

pub struct ExpressionInternal<'a> {
    token: Token<'a>,
    expression: Option<Box<dyn Expression>>,
}

impl<'a> ExpressionInternal<'a> {
    pub fn init(token: Token<'a>, expression: Option<Box<dyn Expression>>) -> Self {
        Self { token, expression }
    }

    pub fn token_literal(&self) -> &str {
        &self.token.literal
    }
}

pub enum Statement<'a> {
    Let(LetInternal<'a>),
    Return(ReturnInternal<'a>),
    Expression(ExpressionInternal<'a>),
}

#[derive(Default)]
pub struct Program<'a> {
    pub statements: Vec<Statement<'a>>,
}

impl Program<'_> {
    pub fn new() -> Self {
        Self { statements: vec![] }
    }
}


impl Debug for Program<'_> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let mut s = String::new();
        for stmt in &self.statements {
            if let Statement::Let(i) = stmt {
                s.push_str("let ");
                s.push_str(&i.name().unwrap().value);
                s.push_str(" = ");
            }
        }
        write!(f, "{}", s)
    }
}

impl PartialEq for Program<'_> {
    fn eq(&self, other: &Self) -> bool {
        for (left, right) in self.statements.iter().zip(other.statements.iter()) {
            if let (Statement::Let(l), Statement::Let(r)) = (left, right) {
                if l.name().unwrap().value != r.name().unwrap().value {
                    return false;
                }
            }
        }
        true
    }
}
