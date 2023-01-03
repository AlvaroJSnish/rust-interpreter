use phf::phf_map;

#[derive(Debug, PartialEq)]
pub struct TokenType(String);

impl TokenType {
    pub fn new(token_type: String) -> TokenType {
        TokenType(token_type)
    }
}

#[derive(strum_macros::Display)]
pub enum Tokens {
    // Keywords
    FUNCTION,
    LET,
    TRUE,
    FALSE,
    IF,
    ELSE,
    RETURN,

    // Delimiters
    COMMA,
    SEMICOLON,
    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE, // Operators
    PLUS,
    MINUS,
    BANG,
    ASTERISK,
    SLASH,
    LT,
    GT,
    EQ,
    NOT_EQ,
    ASSIGN, // Identifiers + literals
    IDENT,
    INT,

    // End of file
    EOF,

    // Error
    ILLEGAL,
}

impl Tokens {
    fn as_str(&self) -> &'static str {
        match self {
            Tokens::FUNCTION => "FUNCTION",
            Tokens::LET => "LET",
            Tokens::TRUE => "TRUE",
            Tokens::FALSE => "FALSE",
            Tokens::IF => "IF",
            Tokens::ELSE => "ELSE",
            Tokens::RETURN => "RETURN",
            Tokens::COMMA => ",",
            Tokens::SEMICOLON => ";",
            Tokens::LPAREN => "(",
            Tokens::RPAREN => ")",
            Tokens::LBRACE => "{",
            Tokens::RBRACE => "}",
            Tokens::PLUS => "+",
            Tokens::MINUS => "-",
            Tokens::BANG => "!",
            Tokens::ASTERISK => "*",
            Tokens::SLASH => "/",
            Tokens::LT => "<",
            Tokens::GT => ">",
            Tokens::EQ => "==",
            Tokens::NOT_EQ => "!=",
            Tokens::ASSIGN => "=",
            Tokens::IDENT => "IDENT",
            Tokens::INT => "INT",
            Tokens::EOF => "EOF",
            Tokens::ILLEGAL => "ILLEGAL",
        }
    }
}

#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: char,
}

impl Token {
    pub fn new(token_type: TokenType, literal: char) -> Token {
        Token {
            token_type,
            literal,
        }
    }
}

// static KEYWORDS: HashMap<&str, Tokens> = HashMap::from([
//     ("fn", Tokens::FUNCTION),
//     ("let", Tokens::LET),
//     ("true", Tokens::TRUE),
//     ("false", Tokens::FALSE),
//     ("if", Tokens::IF),
//     ("else", Tokens::ELSE),
//     ("return", Tokens::RETURN),
// ]);

static KEYWORDS: phf::Map<&'static str, Tokens> = phf_map! {
    "fn" => Tokens::FUNCTION,
    "let" => Tokens::LET,
    "true" => Tokens::TRUE,
    "false" => Tokens::FALSE,
    "if" => Tokens::IF,
    "else" => Tokens::ELSE,
    "return" => Tokens::RETURN,
};

pub fn lookup_ident(ident: &String) -> TokenType {
    if let Some(token) = KEYWORDS.get(&ident[..]) {
        return TokenType(token.as_str().to_string());
    }

    TokenType::new(ident.to_string())
}
