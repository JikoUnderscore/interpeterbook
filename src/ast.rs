// use std::rc::Rc;
use crate::token;

pub trait Node {
    fn token_literal(&self) -> Option<&str>;
    fn string(&self) -> String;
}

pub trait Statement: Node {
    fn statement_node(&self);
}

pub trait Expression: Node {
    fn expression_node(&self);
}

#[derive(Debug)]
pub enum EStatements<'es> {
    LetStatement(LetStatement<'es>),
    ReturnStatement(ReturnStatement<'es>),
}
#[allow(unused)]

pub struct Program<'p> {
    pub statements: Vec<EStatements<'p>>,
}

impl Node for Program<'_> {
    fn token_literal(&self) -> Option<&str> {
        if !self.statements.is_empty() {
            let statement = self.statements.get(0).unwrap();
            return match statement {
                EStatements::LetStatement(a) => a.token_literal(),
                _ => {
                    panic!("wtf");
                }
            };
        } else {
            return None;
        }
    }

    fn string(&self) -> String {
        let mut out = String::new();
        for s in self.statements.iter() {
            match s {
                EStatements::LetStatement(a) => out.push_str(&a.string()),
                EStatements::ReturnStatement(a) => out.push_str(&a.string()),
            }
        }

        return out;
    }
}

#[derive(Debug)]
pub struct LetStatement<'ls> {
    pub token: token::Token<'ls>,
    // pub name: &'ls Identifier,
    pub name: Identifier<'ls>,
    pub value: Option<Identifier<'ls>>, // TODO remove Option
}

impl Statement for LetStatement<'_> {
    fn statement_node(&self) {}
}

impl Node for LetStatement<'_> {
    fn token_literal(&self) -> Option<&str> {
        match self.token {
            token::Token::IDENT(l) => Some(l),
            token::Token::INT(l) => Some(l),
            _ => None,
        }
    }

    fn string(&self) -> String {
        let mut out = String::new();
        out.push_str("let ");
        out.push_str(&self.name.string());
        out.push_str(" = ");

        if let Some(v) = &self.value {
            out.push_str(&v.string());
        }
        out.push(';');


        return out;
    }
}

#[derive(Debug)]
pub struct ReturnStatement<'rs> {
    pub token: token::Token<'rs>,
    // pub return_value: Identifier<'ls>,
}

impl Statement for ReturnStatement<'_> {
    fn statement_node(&self) {}
}
impl Node for ReturnStatement<'_> {
    fn token_literal(&self) -> Option<&str> {
        match self.token {
            token::Token::IDENT(l) => Some(l),
            token::Token::INT(l) => Some(l),
            _ => None,
        }
    }

    fn string(&self) -> String {
        let mut out = String::new();

        out.push_str("return ");
        // out.push_str(self.return_value.string());
        out.push(';');

        return out;
    }
}

#[derive(Debug)]
pub struct Identifier<'i> {
    pub token: token::Token<'i>,
}

impl Expression for Identifier<'_> {
    fn expression_node(&self) {}
}

impl Node for Identifier<'_> {
    fn token_literal(&self) -> Option<&str> {
        match self.token.clone() {
            token::Token::IDENT(l) => Some(l),
            token::Token::INT(l) => Some(l),
            _ => None,
        }
    }

    fn string(&self) -> String {
        match self.token.clone() {
            token::Token::IDENT(l) => l.to_string(),
            _ => panic!("wtf"),
        }
    }
}

pub struct ExpressionStatement<'a> {
    token: token::Token<'a>,
    // expression: Identifier<'ls>,
}

impl Node for ExpressionStatement<'_> {
    fn token_literal(&self) -> Option<&str> {
        match self.token.clone() {
            token::Token::IDENT(l) => Some(l),
            token::Token::INT(l) => Some(l),
            _ => None,
        }
    }

    fn string(&self) -> String {
        // self.expression.string()
        todo!()
        // String::new()
    }
}

impl Statement for ExpressionStatement<'_> {
    fn statement_node(&self) {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::{EStatements, LetStatement, Program, Node};

    #[test]
    fn test_strin() {
        let expect_test = "let myVar = anotherVar;";

        let program = Program {
            statements: vec![EStatements::LetStatement(LetStatement {
                token: crate::token::Token::LET,
                name: super::Identifier {
                    token: crate::token::Token::IDENT("myVar"),
                },
                value: Some(super::Identifier { 
                    token: crate::token::Token::IDENT("anotherVar") 
                }),
            })],
        };

        let program_str = program.string();
        assert_eq!(&program_str, expect_test);

    }
}
