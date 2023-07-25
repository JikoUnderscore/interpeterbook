#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Token<'t> {
	ILLEGAL,
	EOF,

	// Identifiers + literals
	IDENT(&'t str),		//	add, foobar, x, y ...
	INT(&'t str),		// 123456
	// IDENT(String),		//	add, foobar, x, y ...
	// INT(String),		// 123456

	// Operators
	ASSIGN,		// = 
	PLUS,			// +
	MINUS,		// - 
	BANG,			// ! 
	ASTERISK,		// * 
	SLASH,		// / 
	LT,			// < 
	GT,			// > 
	EQ,					// ==
	NOTEQ,				// !=





	// Delimiters
	COMMA,		// ,
	SEMICOLON,	// ;


	LPAREN,		// (
	RPAREN,		// )
	LBRACE,		// {
	RBRACE,		// }

	// Keywords
	FUNCTION,
	LET,
	TRUE,
	FALSE,
	IF,
	ELSE,
	RETURN,


}







pub fn lookup_ident(ident: &[u8]) -> Token {
	match ident {
        b"let" => Token::LET,
	    b"fn" => Token::FUNCTION,
	    b"true" => Token::TRUE,
	    b"false" => Token::FALSE,
	    b"if" => Token::IF,
	    b"else" => Token::ELSE,
	    b"return" => Token::RETURN,
	    // _ =>  Token::IDENT(String::from_utf8_lossy(&ident).to_string()),
	    _ => unsafe { Token::IDENT(std::str::from_utf8_unchecked(ident)) },
	    // _ => unsafe { Token::IDENT(Rc::from(std::str::from_utf8_unchecked(ident)) ) },
	}
}

