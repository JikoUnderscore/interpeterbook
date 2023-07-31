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
pub enum EStatements {
    LetStatement(LetStatement),
    ReturnStatement(ReturnStatement),
}
#[allow(unused)]

pub struct Program {
    pub statements: Vec<EStatements>,
}

impl Node for Program {
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
pub struct LetStatement {
    pub token: token::Token,
    // pub name: &'ls Identifier,
    pub name: Identifier,
    pub value: Option<Identifier>, // TODO remove Option
}

impl Statement for LetStatement {
    fn statement_node(&self) {}
}

impl Node for LetStatement {
    fn token_literal(&self) -> Option<&str> {
        match self.token.clone() {
            token::Token::IDENT(l) => Some({
                unsafe {
                    let a = std::slice::from_raw_parts(l.data_ptr, l.len);

                    std::str::from_utf8_unchecked(a)
                }
            }),
            token::Token::INT(l) => Some({
                unsafe {
                    let a = std::slice::from_raw_parts(l.data_ptr, l.len);

                    std::str::from_utf8_unchecked(a)
                }
            }),
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
pub struct ReturnStatement {
    pub token: token::Token,
    // pub return_value: Identifier<'ls>,
}

impl Statement for ReturnStatement {
    fn statement_node(&self) {}
}
impl Node for ReturnStatement {
    fn token_literal(&self) -> Option<&str> {
        match self.token.clone() {
            token::Token::IDENT(l) => Some({
                unsafe {
                    let a = std::slice::from_raw_parts(l.data_ptr, l.len);

                    std::str::from_utf8_unchecked(a)
                }
            }),
            token::Token::INT(l) => Some({
                unsafe {
                    let a = std::slice::from_raw_parts(l.data_ptr, l.len);

                    std::str::from_utf8_unchecked(a)
                }
            }),
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
pub struct Identifier {
    pub token: token::Token,
}

impl Expression for Identifier {
    fn expression_node(&self) {}
}

impl Node for Identifier {
    fn token_literal(&self) -> Option<&str> {
        match self.token.clone() {
            token::Token::IDENT(l) => Some({
                unsafe {
                    let a = std::slice::from_raw_parts(l.data_ptr, l.len);

                    std::str::from_utf8_unchecked(a)
                }
            }),
            token::Token::INT(l) => Some({
                unsafe {
                    let a = std::slice::from_raw_parts(l.data_ptr, l.len);

                    std::str::from_utf8_unchecked(a)
                }
            }),
            _ => None,
        }
    }

    fn string(&self) -> String {
        match self.token.clone() {
            token::Token::IDENT(l) => l.to_str().to_string(),
            _ => panic!("wtf"),
        }
    }
}

pub struct ExpressionStatement {
    token: token::Token,
    // expression: Identifier<'ls>,
}

impl Node for ExpressionStatement {
    fn token_literal(&self) -> Option<&str> {
        match self.token.clone() {
            token::Token::IDENT(l) => Some({
                unsafe {
                    let a = std::slice::from_raw_parts(l.data_ptr, l.len);

                    std::str::from_utf8_unchecked(a)
                }
            }),
            token::Token::INT(l) => Some({
                unsafe {
                    let a = std::slice::from_raw_parts(l.data_ptr, l.len);

                    std::str::from_utf8_unchecked(a)
                }
            }),
            _ => None,
        }
    }

    fn string(&self) -> String {
        // self.expression.string()
        todo!()
        // String::new()
    }
}

impl Statement for ExpressionStatement {
    fn statement_node(&self) {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::{EStatements, LetStatement, Node, Program};

    #[test]
    fn test_strin() {
        let expect_test = "let myVar = anotherVar;";

        let program = Program {
            statements: vec![EStatements::LetStatement(LetStatement {
                token: crate::token::Token::LET,
                name: super::Identifier {
                    token: crate::token::Token::IDENT("myVar".into()),
                },
                value: Some(super::Identifier {
                    token: crate::token::Token::IDENT("anotherVar".into()),
                }),
            })],
        };

        let program_str = program.string();
        assert_eq!(&program_str, expect_test);
    }
}
