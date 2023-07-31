use crate::{ast, lexer, token};



pub struct Parser {
    pub lexer: lexer::Lexer,
    // pub lexer: & mut lexer::Lexer,
    peek_token: token::Token,
    curent_token: token::Token,
    pub errors: Vec<String>,
}

impl Parser {
    #[allow(unused)]
    pub fn new(mut lexer: lexer::Lexer) -> Self {
        let curent_token = lexer.next_token();
        let peek_token = lexer.next_token();

        return Parser {
            lexer,
            peek_token,
            curent_token,
            errors: Vec::new(),
        };
    }

    pub fn next_token(& mut self) {
        self.curent_token = self.peek_token.clone(); // copy
        self.peek_token = self.lexer.next_token(); // copy
    }

    // #[allow(unused)]
    // pub unsafe fn next_token_unsafe(p: *mut Parser) {
    //     (*p).curent_token = (*p).peek_token.clone(); // copy
    //     (*p).peek_token = (*p).lexer.next_token(); // copy
    // }

    #[allow(unused)]
    pub fn parse_program(&mut self) -> Result<ast::Program, ()> {
        let mut program = ast::Program {
            statements: Vec::new(),
        };
        // let p: *mut Parser = &mut self as *mut _;

        while self.curent_token.clone() != token::Token::EOF {
        // while self.curent_token != token::Token::EOF {
            // if let Some(stmt) = unsafe { Parser::parse_statement_unsafe(p) } {
            if let Some(stmt) = self.parse_statement() {
                program.statements.push(stmt);
            }
            self.next_token();
            // unsafe { Parser::next_token_unsafe(p) };
        }

        return Ok(program);
    }

    fn parse_statement(&mut self) -> Option<ast::EStatements> {
        match self.curent_token {
            token::Token::LET => self.parse_let_statement(),
            token::Token::RETURN => self.parse_return_statement(),
            _ => None,
        }
    }

    // unsafe fn parse_statement_unsafe(p: *mut Parser) -> Option<ast::EStatements> {
    //     match (*p).curent_token {
    //         token::Token::LET => (*p).parse_let_statement(),
    //         token::Token::RETURN => (*p).parse_return_statement(),
    //         _ => None,
    //     }
    // }

    fn parse_return_statement(&'_ mut self) -> Option<ast::EStatements> {
        let stmt_token = self.curent_token.clone(); // copy

        let stmt = ast::ReturnStatement {
            token: stmt_token,
            // name: ast::Identifier { token },
        };


        // TODO: We're skipping the expressions until we
        // encounter a semicolon
        while self.curent_token != token::Token::SEMICOLON {
            self.next_token();
            // unsafe { Parser::next_token_unsafe(self as *mut _) };
        }

        return Some(ast::EStatements::ReturnStatement(stmt));
    }

    fn parse_let_statement(&'_ mut self) -> Option<ast::EStatements> {
        let stmt_token = self.curent_token.clone(); // copy

        if let token::Token::IDENT(ident) = self.peek_token.clone() {
            if !self.expect_peek(token::Token::IDENT(ident)) {
                return None;
            }
        }

        // println!("cat");
        let token = self.curent_token.clone();

        let stmt = ast::LetStatement {
            token: stmt_token,
            name: ast::Identifier { token },
            value: None,
        };

        if !self.expect_peek(token::Token::ASSIGN) {
            return None;
        }

        // TODO: We're skipping the expressions until we
        // encounter a semicolon
        while self.curent_token != token::Token::SEMICOLON {
            self.next_token();
            // unsafe { Parser::next_token_unsafe(self as *mut _) };
        }

        return Some(ast::EStatements::LetStatement(stmt));
    }

    fn expect_peek(&mut self, t: token::Token) -> bool {
        if self.peek_token == t {
            self.next_token();
            // unsafe { Parser::next_token_unsafe(self as *mut _) };

            return true;
        } else {
            self.peek_error(t);
            return false;
        }
    }

    pub fn peek_error(&mut self, token: token::Token) {
        let msg = format!(
            "Expecter next token to be {:?}, got {:?} instead",
            token, self.peek_token
        );
        self.errors.push(msg);
    }
}

#[cfg(test)]
mod tests {

    use crate::{
        ast::{self, Node},
        lexer, token,
    };

    use super::Parser;

    fn check_parser_errors(p: &Parser) {
        if p.errors.is_empty() {
            return;
        }

        eprintln!("parser has {} errors", p.errors.len());
        for er in p.errors.iter() {
            eprintln!("parser error {:?}", er);
        }
        panic!("failed with above errors");
    }

    #[test]
    fn parser_test() {
        fn test_statment(statement: &ast::EStatements, name: &str) {
            match statement {
                ast::EStatements::LetStatement(letstmt) => {
                    if letstmt.token_literal() != None {
                        panic!("token literal not `None`");
                    }

                    // assert_eq!(
                    //     letstmt.name.value, name,
                    //     "letstmt.name.value not `{}` got `{}`",
                    //     name, letstmt.name.value
                    // );

                    assert_eq!(
                        letstmt.name.token_literal().unwrap(),
                        name,
                        "letstmt.name.token_literal().unwrap() not `{}` got `{}`",
                        name,
                        letstmt.name.token_literal().unwrap()
                    );
                }
                _ => {
                    panic!("not a let statement");
                }
            }
        }

        let input = "
let x = 5;
let y = 10;
let foobar = 838383;
";

        let lexer = lexer::Lexer::new(input.as_bytes());
        let mut parser = Parser::new(lexer);
        let program;
        
        program = parser.parse_program().expect("wtf");

        check_parser_errors(&parser);

        if program.statements.len() != 3 {
            panic!(
                "program.Statements does not contain 3 statements. got={}",
                program.statements.len()
            );
        }

        let expected = ["x", "y", "foobar"];

        for (i, expected_name) in expected.iter().enumerate() {
            let stmt = program.statements.get(i).unwrap();
            // dbg!(stmt);
            test_statment(stmt, expected_name);
        }
    }

    #[test]
    fn return_statems() {
        let input = "
return 5;
return 10;
return 993322;
";
        let lexer = lexer::Lexer::new(input.as_bytes());
        let mut parser = Parser::new(lexer);
        let program;

        program = parser.parse_program().expect("wtf");
        check_parser_errors(&parser);

                if program.statements.len() != 3 {
            panic!(
                "program.Statements does not contain 3 statements. got={}",
                program.statements.len()
            );
        }

        for stmt in program.statements.iter() {
            match stmt {
                ast::EStatements::ReturnStatement(a) => {
                    // dbg!(a);
                    assert_eq!(a.token, token::Token::RETURN);
                },
                _ => eprintln!("stmt not ReturnStatement. got {:?}", stmt),
            }



        }
    }
}
