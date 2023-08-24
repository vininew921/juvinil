use super::{
    comparators::Comparator, jv_types::JvType, keyword::Keyword, operators::Operator,
    regex_token::RegexToken, symbols::Symbol,
};

#[derive(Debug, Clone)]
pub enum TokenType {
    KEYWORD,
    OPERATOR,
    TYPE,
    SYMBOL,
    COMPARATOR,
    ID,
    STRING,
    NUMBER,
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

    pub fn from_keyword(value: &Keyword) -> Self {
        Token::new(TokenType::KEYWORD, format!("{:?}", value))
    }

    pub fn from_operator(value: &Operator) -> Self {
        Token::new(TokenType::OPERATOR, format!("{:?}", value))
    }

    pub fn from_type(value: &JvType) -> Self {
        Token::new(TokenType::TYPE, format!("{:?}", value))
    }

    pub fn from_symbol(value: &Symbol) -> Self {
        Token::new(TokenType::SYMBOL, format!("{:?}", value))
    }

    pub fn from_comparator(value: &Comparator) -> Self {
        Token::new(TokenType::COMPARATOR, format!("{:?}", value))
    }

    pub fn from_id(value: &str) -> Self {
        Token::new(TokenType::ID, value.into())
    }

    pub fn from_string(value: &str) -> Self {
        Token::new(TokenType::STRING, String::from(value).replace(r#"""#, ""))
    }

    pub fn from_number(value: &str) -> Self {
        Token::new(TokenType::NUMBER, value.into())
    }

    pub fn from_regex_token(rt: &RegexToken, value: &str) -> Self {
        match rt.token_type {
            TokenType::ID => Token::from_id(value),
            TokenType::STRING => Token::from_string(value),
            TokenType::NUMBER => Token::from_number(value),
            _ => panic!("This shouldn't be possible xdd"),
        }
    }
}

impl std::fmt::Debug for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<{:?} {}>", self.token_type, self.value)
    }
}
