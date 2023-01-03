use crate::token::{lookup_ident, Token, TokenType, Tokens};

pub struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
    ch: char,
}

impl Lexer {
    pub fn new(input: String) -> Lexer {
        let mut l = Lexer {
            input,
            position: 0,
            read_position: 0,
            ch: '\0',
        };
        l.read_char();
        l
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = '\0';
        } else {
            self.ch = self.input.chars().nth(self.read_position).unwrap();
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    fn peek_char(&self) -> char {
        if self.read_position >= self.input.len() {
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

    fn read_number(&mut self) -> String {
        let position = self.position;
        while self.ch.is_numeric() {
            self.read_char();
        }
        self.input[position..self.position].to_string()
    }

    fn skip_whitespace(&mut self) {
        while self.ch.is_whitespace() {
            self.read_char();
        }
    }

    pub fn next_token(&mut self) -> Token {
        let tok: Token;
        self.skip_whitespace();

        match self.ch {
            '=' => {
                if self.peek_char() == '=' {
                    let ch = self.ch;
                    self.read_char();
                    let literal = format!("{}{}", ch, self.ch).chars().next().unwrap();
                    tok = Token::new(TokenType::new(Tokens::EQ.to_string()), literal);
                } else {
                    tok = Token::new(TokenType::new(Tokens::ASSIGN.to_string()), self.ch);
                }
            }
            ';' => tok = Token::new(TokenType::new(Tokens::SEMICOLON.to_string()), self.ch),
            ',' => tok = Token::new(TokenType::new(Tokens::COMMA.to_string()), self.ch),
            '(' => tok = Token::new(TokenType::new(Tokens::LPAREN.to_string()), self.ch),
            ')' => tok = Token::new(TokenType::new(Tokens::RPAREN.to_string()), self.ch),
            '+' => tok = Token::new(TokenType::new(Tokens::PLUS.to_string()), self.ch),
            '{' => tok = Token::new(TokenType::new(Tokens::LBRACE.to_string()), self.ch),
            '}' => tok = Token::new(TokenType::new(Tokens::RBRACE.to_string()), self.ch),
            '-' => tok = Token::new(TokenType::new(Tokens::MINUS.to_string()), self.ch),
            '!' => {
                if self.peek_char() == '=' {
                    let ch = self.ch;
                    self.read_char();
                    let literal = format!("{}{}", ch, self.ch).chars().next().unwrap();
                    tok = Token::new(TokenType::new(Tokens::NOT_EQ.to_string()), literal);
                } else {
                    tok = Token::new(TokenType::new(Tokens::BANG.to_string()), self.ch);
                }
            }
            '/' => tok = Token::new(TokenType::new(Tokens::SLASH.to_string()), self.ch),
            '*' => tok = Token::new(TokenType::new(Tokens::ASTERISK.to_string()), self.ch),
            '<' => tok = Token::new(TokenType::new(Tokens::LT.to_string()), self.ch),
            '>' => tok = Token::new(TokenType::new(Tokens::GT.to_string()), self.ch),
            '\0' => tok = Token::new(TokenType::new(Tokens::EOF.to_string()), self.ch),
            _ => {
                if self.ch.is_alphabetic() {
                    let literal = self.read_identifier();
                    let token_type = lookup_ident(&literal);
                    return Token::new(token_type, literal.chars().next().unwrap());
                } else if self.ch.is_numeric() {
                    let literal = self.read_number();
                    return Token::new(
                        TokenType::new(Tokens::INT.to_string()),
                        literal.chars().next().unwrap(),
                    );
                } else {
                    tok = Token::new(TokenType::new(Tokens::ILLEGAL.to_string()), self.ch);
                }
            }
        }

        self.read_char();
        tok
    }
}
