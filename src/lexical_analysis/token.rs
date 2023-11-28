use super::regex_token::RegexToken;

pub const KEYWORDS: [&str; 11] = [
    "func", "return", "if", "else", "for", "do", "while", "break", "continue", "true", "false",
];

pub const OPERATORS: [&str; 13] = [
    "=", "+", "-", "*", "/", "%", "!", "&", "|", "++", "--", "+=", "-=",
];

pub const JV_TYPES: [&str; 4] = ["void", "int", "boolean", "string"];

pub const SYMBOLS: [&str; 8] = [";", "(", ")", "[", "]", "{", "}", ","];

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
    pub file_line: usize,
}

impl Token {
    pub fn new(token_type: TokenType, value: String, file_line: usize) -> Self {
        Token {
            token_type,
            value,
            file_line,
        }
    }

    pub fn new_keyword(value: String, file_line: usize) -> Self {
        Token::new(TokenType::KEYWORD, value, file_line)
    }

    pub fn new_operator(value: String, file_line: usize) -> Self {
        Token::new(TokenType::OPERATOR, value, file_line)
    }

    pub fn new_type(value: String, file_line: usize) -> Self {
        Token::new(TokenType::TYPE, value, file_line)
    }

    pub fn new_symbol(value: String, file_line: usize) -> Self {
        Token::new(TokenType::SYMBOL, value, file_line)
    }

    pub fn new_comparator(value: String, file_line: usize) -> Self {
        Token::new(TokenType::COMPARATOR, value, file_line)
    }

    pub fn new_id(value: String, file_line: usize) -> Self {
        Token::new(TokenType::ID, value, file_line)
    }

    pub fn new_string(value: String, file_line: usize) -> Self {
        Token::new(TokenType::STRING, value.replace('"', ""), file_line)
    }

    pub fn new_number(value: String, file_line: usize) -> Self {
        Token::new(TokenType::NUMBER, value, file_line)
    }

    pub fn from_regex_token(rt: &RegexToken, value: &str, file_line: usize) -> Self {
        match rt.token_type {
            TokenType::ID => Token::new_id(value.into(), file_line),
            TokenType::STRING => Token::new_string(value.into(), file_line),
            TokenType::NUMBER => Token::new_number(value.into(), file_line),
            _ => panic!("This shouldn't be possible xdd"),
        }
    }

    pub fn eof(file_line: usize) -> Self {
        Token::new(TokenType::EOF, "".into(), file_line)
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
