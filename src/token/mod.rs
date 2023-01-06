#[derive(Debug, PartialEq, Eq)]
pub enum TokenType {
    // Keywords
    If,
    Else,
    Function,
    Var,
    Let,
    Const,
    Return,
    True,
    False,
    Null,
    Undefined,
    // Operators
    Plus,
    Minus,
    Multiply,
    Divide,
    Modulus,
    Equal,
    NotEqual,
    GreaterThan,
    GreaterThanEqual,
    LessThan,
    LessThanEqual,
    And,
    Or,
    Not,
    Assign,
    // Punctuation
    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
    LeftBrace,
    RightBrace,
    Comma,
    Semicolon,
    Colon,
    // Literals
    Number(i64),
    String(String),
    // Other
    Identifier(String),
    EOF,
    Illegal,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
}

impl Token {
    pub fn new(token_type: TokenType, literal: String) -> Token {
        Token {
            token_type,
            literal,
        }
    }
}

pub fn lookup_ident(ident: &str) -> TokenType {
    match ident {
        "fn" => TokenType::Function,
        "let" => TokenType::Let,
        "true" => TokenType::True,
        "false" => TokenType::False,
        "if" => TokenType::If,
        "else" => TokenType::Else,
        "return" => TokenType::Return,
        "const" => TokenType::Const,
        "var" => TokenType::Var,
        "null" => TokenType::Null,
        "undefined" => TokenType::Undefined,
        _ => TokenType::Identifier(ident.to_string()),
    }
}
