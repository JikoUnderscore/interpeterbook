use std::{io::{Write, BufRead}, print};

use crate::{lexer, token};

pub const PROMPT: &[u8] = b">>";

pub fn start<R: std::io::Read, W: std::io::Write>(arg_in: R,  out_arg: W) {
    let mut scanner = std::io::BufReader::new(arg_in);

    let mut buff_out = std::io::BufWriter::new(out_arg);

    let mut buff = String::with_capacity(100);

    loop {
        print!("{:?}", PROMPT);
        buff_out.flush().unwrap();
        

        {
            if let Ok(scanned) = scanner.read_line(&mut buff) {
                if scanned == 0 {
                	panic!("cat");
                }
            }
        }

        let mut lexer = lexer::Lexer::new(buff.as_bytes());

        let mut tok = lexer.next_token();
        while tok != token::Token::EOF {
            let format_str = format!("{:?}\n", tok);
            buff_out.write_all(format_str.as_bytes()).unwrap();

            tok = lexer.next_token();
        }
    }
}
