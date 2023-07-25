// use std::rc::Rc;

use crate::token;

#[derive(Debug)]
pub struct Lexer {
    input: Vec<u8>,
    position: usize,
    read_positoin: usize,
    curent_char: u8,
}

impl Lexer {
    pub fn new(input: &[u8]) -> Self {
        let mut l = Self {
            input: input.to_owned(),
            position: 0,
            read_positoin: 0,
            curent_char: Default::default(),
        };

        l.read_char();
        return l;
    }

    fn peek_char(&self) -> u8{
        if self.read_positoin >= self.input.len() {
            return b'\0'; 
        } else {
            let chr = self.input[self.read_positoin];
            return chr;
        }
    }

    pub fn read_char(&mut self) {
        if self.read_positoin >= self.input.len() {
            self.curent_char = b'\0';
        } else {
            self.curent_char = self.input[self.read_positoin];
        }

        self.position = self.read_positoin;
        self.read_positoin += 1;
    }

    pub fn next_token(&mut self) -> token::Token {
    	let tok = {
            self.skip_white_space();

    		match self.curent_char {
    			b'=' => {
                    if self.peek_char() == b'=' {
                        // let chr = self.curent_char;
                        self.read_char();
                        token::Token::EQ
                    } else {
                        token::Token::ASSIGN
                    }
                },
    			b';' => token::Token::SEMICOLON,
    			b'(' => token::Token::LPAREN,
    			b')' => token::Token::RPAREN,
    			b',' => token::Token::COMMA,
    			b'+' => token::Token::PLUS,
                b'-' => token::Token::MINUS,
                b'!' => {
                    if self.peek_char() == b'=' {
                        // let chr = self.curent_char;
                        self.read_char();
                        token::Token::NOTEQ
                    } else {
                        token::Token::BANG
                    }
                },
                b'/' => token::Token::SLASH,
                b'*' => token::Token::ASTERISK,
                b'<' => token::Token::LT,
                b'>' => token::Token::GT,

    			b'{' => token::Token::LBRACE,
    			b'}' => token::Token::RBRACE,
				b'\0'=> token::Token::EOF,
    			_ => {
    				if Lexer::is_letter(self.curent_char) {
						let literal = self.read_identifier();
						return token::lookup_ident(literal);
				    } else if self.curent_char.is_ascii_digit() {
                        // return unsafe {token::Token::INT(std::str::from_utf8_unchecked(self.read_number()))}; 
                        return token::Token::INT(self.read_number()); 
                    } else {
    				    token::Token::ILLEGAL
    				}
    			},
    		}
    	};

    	self.read_char();
    	return tok;
    }

    pub fn skip_white_space(&mut self) {
        while self.curent_char == b' ' || self.curent_char == b'\t' || self.curent_char == b'\n' || self.curent_char == b'\r' {
            self.read_char();
        }
    }

    pub fn read_identifier(&mut self) -> &[u8] {
		let position = self.position;
		while Lexer::is_letter(self.curent_char) {
		    self.read_char();
		}
		return &self.input[position..self.position];
        // return String::from_utf8_lossy(&self.input[position..self.position]).to_string();
        
    }

    fn is_letter(curent_char: u8) -> bool{
    	return curent_char.is_ascii_alphabetic() || curent_char == b'_';
    }

    fn read_number(&mut self) -> &str {
        let position = self.position;
        while self.curent_char.is_ascii_digit() {
            self.read_char();
        }

        return unsafe {std::str::from_utf8_unchecked(&self.input[position..self.position])};
        // return unsafe { Rc::from(std::str::from_utf8_unchecked(&self.input[position..self.position]) )};

    }


}

#[cfg(test)]
mod tests {
    use crate::token;

    use super::Lexer;

    #[test]
    fn lexer_basic() {
    	let input = "=+(){},;";

    	let expected = [
    		token::Token::ASSIGN,
    		token::Token::PLUS,
    		token::Token::LPAREN,
    		token::Token::RPAREN,
    		token::Token::LBRACE,
    		token::Token::RBRACE,
    		token::Token::COMMA,
    		token::Token::SEMICOLON,
    		token::Token::EOF,
    	];

    	let mut lexer = Lexer::new(input.as_bytes());

    	for expected_token in expected.iter() {
    		let tok = lexer.next_token();
    		assert_eq!(tok, *expected_token);
    	}
    }

    #[test]
    fn lexer_big() {
    	let input = "let five = 5;
let ten = 10;

let add = fn(x, y) {
	x + y;
};

let result = add(five, ten);
";

    	let expected = [
    		token::Token::LET,
    		token::Token::IDENT("five".into()),
    		token::Token::ASSIGN,
    		token::Token::INT("5".into()),
    		token::Token::SEMICOLON,
    		token::Token::LET,
    		token::Token::IDENT("ten".into()),
    		token::Token::ASSIGN,
    		token::Token::INT("10".into()),
    		token::Token::SEMICOLON,
    		token::Token::LET,
    		token::Token::IDENT("add".into()),
    		token::Token::ASSIGN,
    		token::Token::FUNCTION,
    		token::Token::LPAREN,
    		token::Token::IDENT("x".into()),
    		token::Token::COMMA,
    		token::Token::IDENT("y".into()),
    		token::Token::RPAREN,
    		token::Token::LBRACE,
    		token::Token::IDENT("x".into()),
            token::Token::PLUS,
            token::Token::IDENT("y".into()),
            token::Token::SEMICOLON,
            token::Token::RBRACE,
            token::Token::SEMICOLON,
            token::Token::LET,
            token::Token::IDENT("result".into()),
            token::Token::ASSIGN,
            token::Token::IDENT("add".into()),
            token::Token::LPAREN,
            token::Token::IDENT("five".into()),
            token::Token::COMMA,
            token::Token::IDENT("ten".into()),
    		token::Token::RPAREN,
    		token::Token::SEMICOLON,
    		token::Token::EOF,
    	];

    	let mut lexer = Lexer::new(input.as_bytes());

    	for expected_token in expected.iter() {
    		let tok = lexer.next_token();
    		assert_eq!(tok, *expected_token);
    	}
    }


    #[test]
    fn lexer_new_stuff() {
        let input = "let five = 5;
let ten = 10;

let add = fn(x, y) {
    x + y;
};

let result = add(five, ten);
!-/*5;
5 < 10 > 5;

if (5 < 10){
    return true;
} else {
    return false;
}

10 == 10;
10 != 9;
";

        let expected = [
            token::Token::LET,
            token::Token::IDENT("five".into()),
            token::Token::ASSIGN,
            token::Token::INT("5".into()),
            token::Token::SEMICOLON,
            token::Token::LET,
            token::Token::IDENT("ten".into()),
            token::Token::ASSIGN,
            token::Token::INT("10".into()),
            token::Token::SEMICOLON,
            token::Token::LET,
            token::Token::IDENT("add".into()),
            token::Token::ASSIGN,
            token::Token::FUNCTION,
            token::Token::LPAREN,
            token::Token::IDENT("x".into()),
            token::Token::COMMA,
            token::Token::IDENT("y".into()),
            token::Token::RPAREN,
            token::Token::LBRACE,
            token::Token::IDENT("x".into()),
            token::Token::PLUS,
            token::Token::IDENT("y".into()),
            token::Token::SEMICOLON,
            token::Token::RBRACE,
            token::Token::SEMICOLON,
            token::Token::LET,
            token::Token::IDENT("result".into()),
            token::Token::ASSIGN,
            token::Token::IDENT("add".into()),
            token::Token::LPAREN,
            token::Token::IDENT("five".into()),
            token::Token::COMMA,
            token::Token::IDENT("ten".into()),
            token::Token::RPAREN,
            token::Token::SEMICOLON,
            token::Token::BANG,
            token::Token::MINUS,
            token::Token::SLASH,
            token::Token::ASTERISK,
            token::Token::INT("5".into()),
            token::Token::SEMICOLON,
            token::Token::INT("5".into()),
            token::Token::LT,
            token::Token::INT("10".into()),
            token::Token::GT,
            token::Token::INT("5".into()),
            token::Token::SEMICOLON,
            token::Token::IF,
            token::Token::LPAREN,
            token::Token::INT("5".into()),
            token::Token::LT,
            token::Token::INT("10".into()),
            token::Token::RPAREN,
            token::Token::LBRACE,
            token::Token::RETURN,
            token::Token::TRUE,
            token::Token::SEMICOLON,
            token::Token::RBRACE,
            token::Token::ELSE,
            token::Token::LBRACE,
            token::Token::RETURN,
            token::Token::FALSE,
            token::Token::SEMICOLON,
            token::Token::RBRACE,
            token::Token::INT("10".into()),
            token::Token::EQ,
            token::Token::INT("10".into()),
            token::Token::SEMICOLON,
            token::Token::INT("10".into()),
            token::Token::NOTEQ,
            token::Token::INT("9".into()),
            token::Token::SEMICOLON,
            token::Token::EOF,
        ];

        let mut lexer = Lexer::new(input.as_bytes());

        for expected_token in expected.iter() {
            let tok = lexer.next_token();
            // println!("{:?} == {:?}", tok, expected_token);
            assert_eq!(tok, *expected_token);
        }
    }
}
