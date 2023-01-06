use std::io::{stdout, Write};

use crate::{lexer, token};

pub fn start() {
    let mut buffer = String::new();
    loop {
        buffer.clear();
        print!(">> ");
        stdout().flush().unwrap();
        std::io::stdin().read_line(&mut buffer).unwrap();

        let mut lexer = lexer::Lexer::new(&buffer);
        let mut token = lexer.next_token();

        while token.token_type != token::TokenType::EOF {
            println!(
                "Type: {:?} - Literal: {:?}",
                token.token_type, token.literal
            );
            token = lexer.next_token();
        }
    }
}
