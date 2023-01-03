static PROMPT: &str = ">> ";

use std::io::prelude::*;
use std::io::BufReader;
use std::io::BufWriter;

use crate::lexer::Lexer;
use crate::token::TokenType;
use crate::token::Tokens;

pub fn start(input: &mut impl Read, out: &mut impl Write) {
    println!("Hello! This is the Monkey programming language!");
    println!("Feel free to type in commands");

    let mut scanner = BufReader::new(input);
    // let mut out = BufWriter::new(out);
    let mut line = String::new();

    loop {
        print!("{}", PROMPT);
        let scanned = scanner.read_line(&mut line).is_ok();
        if !scanned {
            return;
        }
        let mut l = Lexer::new(line.clone());
        loop {
            let tok = l.next_token();

            if tok.token_type == TokenType::new(Tokens::EOF.to_string()) {
                break;
            }

            println!("{:?}\n", tok);
        }

        line.clear();
    }
}
