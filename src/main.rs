#![allow(non_snake_case)]
#![allow(clippy::needless_return)]
#![allow(clippy::upper_case_acronyms)]

mod ast;
mod lexer;
mod parser;
mod repl;
mod token;


// page 51
// --nocapture
fn main() {
    println!("Hello THERE! This is the Monkey programming language!");
    println!("Feel free to type in commands");
    repl::start(std::io::stdin(), std::io::stdout());
}

