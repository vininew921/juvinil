use super::regex_token::RegexToken;

pub const KEYWORDS: [&str; 12] = [
    "func", "return", "if", "else", "for", "while", "break", "continue", "switch", "case", "true",
    "false",
];

pub const OPERATORS: [&str; 13] = [
    "=", "+", "-", "*", "/", "%", "!", "&", "|", "++", "--", "+=", "-=",
];

pub const JV_TYPES: [&str; 6] = ["void", "int", "float", "boolean", "char", "string"];

pub const SYMBOLS: [&str; 7] = [";", "(", ")", "[", "]", "{", "}"];

pub const COMPARATORS: [&str; 8] = ["&&", "||", "==", "!=", "<", ">", ">=", "<="];

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    KEYWORD,
    OPERATOR,
    TYPE,
    SYMBOL,
    COMPARATOR,
    ID,
    STRING,
    NUMBER,
    EOF,
}

#[derive(Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub value: String,
}

impl Token {
    pub fn new(token_type: TokenType, value: String) -> Self {
        Token { token_type, value }
    }

    pub fn new_keyword(value: String) -> Self {
        Token::new(TokenType::KEYWORD, value)
    }

    pub fn new_operator(value: String) -> Self {
        Token::new(TokenType::OPERATOR, value)
    }

    pub fn new_type(value: String) -> Self {
        Token::new(TokenType::TYPE, value)
    }

    pub fn new_symbol(value: String) -> Self {
        Token::new(TokenType::SYMBOL, value)
    }

    pub fn new_comparator(value: String) -> Self {
        Token::new(TokenType::COMPARATOR, value)
    }

    pub fn new_id(value: String) -> Self {
        Token::new(TokenType::ID, value)
    }

    pub fn new_string(value: String) -> Self {
        Token::new(TokenType::STRING, value.replace('"', ""))
    }

    pub fn new_number(value: String) -> Self {
        Token::new(TokenType::NUMBER, value)
    }

    pub fn from_regex_token(rt: &RegexToken, value: &str) -> Self {
        match rt.token_type {
            TokenType::ID => Token::new_id(value.into()),
            TokenType::STRING => Token::new_string(value.into()),
            TokenType::NUMBER => Token::new_number(value.into()),
            _ => panic!("This shouldn't be possible xdd"),
        }
    }

    pub fn eof() -> Self {
        Token::new(TokenType::EOF, "".into())
    }

    pub fn values(&self) -> (TokenType, &str) {
        (self.token_type.clone(), self.value.as_str())
    }
}

impl std::fmt::Debug for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<{:?} {}>", self.token_type, self.value)
    }
}
