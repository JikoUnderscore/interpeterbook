use std::fmt::Debug;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Token {
    ILLEGAL,
    EOF,

    // Identifiers + literals
    // IDENT(&'t str),		//	add, foobar, x, y ...
    // INT(&'t str),		// 123456
    IDENT(RawSlice), //	add, foobar, x, y ...
    INT(RawSlice),   // 123456

    // Operators
    ASSIGN,   // =
    PLUS,     // +
    MINUS,    // -
    BANG,     // !
    ASTERISK, // *
    SLASH,    // /
    LT,       // <
    GT,       // >
    EQ,       // ==
    NOTEQ,    // !=

    // Delimiters
    COMMA,     // ,
    SEMICOLON, // ;

    LPAREN, // (
    RPAREN, // )
    LBRACE, // {
    RBRACE, // }

    // Keywords
    FUNCTION,
    LET,
    TRUE,
    FALSE,
    IF,
    ELSE,
    RETURN,
}

#[derive(Clone, Eq)]
pub struct RawSlice {
    pub data_ptr: *const u8,
    pub len: usize,
}

impl RawSlice {
    #[inline]
    pub fn to_str(&self) -> &str {
        unsafe {
            let a = std::slice::from_raw_parts(self.data_ptr, self.len);

            return std::str::from_utf8_unchecked(a);
        }
    }
}

impl PartialEq for RawSlice {
    fn eq(&self, other: &Self) -> bool {
        let this = unsafe{std::slice::from_raw_parts(self.data_ptr, self.len)};
        let other_s = unsafe{std::slice::from_raw_parts(other.data_ptr, other.len)};

        return  this == other_s;
    }
}

impl Debug for RawSlice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let this = unsafe{std::slice::from_raw_parts(self.data_ptr, self.len)};
        let this_str = unsafe{std::str::from_utf8_unchecked(this)};


        // f.debug_struct("RawSlice").field("data_ptr", &self.data_ptr).field("len", &self.len).finish()
        f.debug_list().entries(this_str.chars()).finish()
    }
}

impl From<&str> for RawSlice {
    fn from(value: &str) -> Self {
        let len = value.len();
        let data_ptr = value.as_ptr();

        return RawSlice { data_ptr, len };
    }
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
        _ => Token::IDENT(RawSlice { data_ptr: ident.as_ptr(), len: ident.len() }),
        // _ => unsafe { Token::IDENT(Rc::from(std::str::from_utf8_unchecked(ident)) ) },
    }
}
