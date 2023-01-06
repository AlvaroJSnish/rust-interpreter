use crate::token::{lookup_ident, Token, TokenType};

#[derive(Debug)]
pub struct Lexer<'a> {
    input: &'a str,
    position: usize,
    read_position: usize,
    ch: char,
}

impl Lexer<'_> {
    pub fn new(input: &str) -> Lexer {
        Lexer {
            input,
            position: 0,
            read_position: 0,
            ch: '\0',
        }
    }

    fn read_char(&mut self) {
        if self.read_position > self.input.len() {
            self.ch = '\0';
        } else if let Some(ch) = self.input.chars().nth(self.read_position) {
            self.ch = ch;
        }

        self.position = self.read_position;
        self.read_position += 1;
    }

    fn skip_whitespace(&mut self) {
        while self.ch.is_whitespace() {
            self.read_char();
        }
    }

    fn peek_char(&self) -> char {
        if self.read_position > self.input.len() {
            '\0'
        } else {
            self.input.chars().nth(self.read_position).unwrap()
        }
    }

    fn read_identifier(&mut self) -> String {
        let position = self.position;
        while self.ch.is_alphabetic() {
            self.read_char();
        }

        self.input[position..self.position].to_string()
    }

    pub fn next_token(&mut self) -> Token {
        let mut token = Token::new(TokenType::Illegal, self.ch.to_string());

        self.read_char();
        self.skip_whitespace();

        match self.ch {
            '=' => {
                if self.peek_char() == '=' {
                    let ch = self.ch;
                    self.read_char();
                    let literal = format!("{}{}", ch, self.ch);
                    token = Token::new(TokenType::Equal, literal);
                    token
                } else {
                    token = Token::new(TokenType::Assign, self.ch.to_string());
                    token
                }
            }
            '!' => {
                if self.peek_char() == '=' {
                    let ch = self.ch;
                    self.read_char();
                    let literal = format!("{}{}", ch, self.ch);
                    token = Token::new(TokenType::NotEqual, literal);
                    token
                } else {
                    token = Token::new(TokenType::Not, self.ch.to_string());
                    token
                }
            }
            '+' => Token::new(TokenType::Plus, self.ch.to_string()),
            '-' => Token::new(TokenType::Minus, self.ch.to_string()),
            '*' => Token::new(TokenType::Multiply, self.ch.to_string()),
            '/' => Token::new(TokenType::Divide, self.ch.to_string()),
            '%' => Token::new(TokenType::Modulus, self.ch.to_string()),
            '<' => Token::new(TokenType::LessThan, self.ch.to_string()),
            '>' => Token::new(TokenType::GreaterThan, self.ch.to_string()),
            ';' => Token::new(TokenType::Semicolon, self.ch.to_string()),
            ',' => Token::new(TokenType::Comma, self.ch.to_string()),
            '(' => Token::new(TokenType::LeftParen, self.ch.to_string()),
            ')' => Token::new(TokenType::RightParen, self.ch.to_string()),
            '{' => Token::new(TokenType::LeftBrace, self.ch.to_string()),
            '}' => Token::new(TokenType::RightBrace, self.ch.to_string()),
            '[' => Token::new(TokenType::LeftBracket, self.ch.to_string()),
            ']' => Token::new(TokenType::RightBracket, self.ch.to_string()),
            ':' => Token::new(TokenType::Colon, self.ch.to_string()),
            '"' => {
                let position = self.position + 1;
                while self.peek_char() != '"' {
                    self.read_char();
                }

                self.read_char();
                let literal = self.input[position..self.position].to_string();
                token = Token::new(TokenType::String(literal), self.ch.to_string());
                token
            }
            '\0' => Token::new(TokenType::EOF, self.ch.to_string()),
            _ => {
                if self.ch.is_alphabetic() {
                    token.literal = self.read_identifier();
                    token.token_type = lookup_ident(&token.literal);
                    token
                } else if self.ch.is_numeric() {
                    token.literal = self.ch.to_string();
                    token.token_type = TokenType::Number(token.literal.parse::<i64>().unwrap());
                    token

                    // let position = self.position;
                    // while self.ch.is_numeric() {
                    //     self.read_char();
                    // }

                    // let literal = self.input[position..self.position].to_string();

                    // token = Token::new(TokenType::Number(literal.parse::<i64>().unwrap()), literal);

                    // token
                } else {
                    token = Token::new(TokenType::Illegal, self.ch.to_string());
                    token
                }
            }
        }
    }
}
