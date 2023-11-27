use crate::lexical_analysis::token::{Token, TokenType};

#[derive(Debug, thiserror::Error)]
pub enum JuvinilError {
    #[error("Couldn't match token {0} to any defined expression - line {1}")]
    LexicalError(String, usize),

    #[error("Syntax error - Expecting <{0:?} {1}>, found {2:?}")]
    SyntaxError(TokenType, String, Token),

    #[error("Duplicate Variable - Variable {0} was already declared in the current scope")]
    DuplicateVariable(String),

    #[error("Undeclared Variable - Variable {0} was not declared in the current scope")]
    UndeclaredVariable(String),

    #[error("Parsing error")]
    ParsingError,

    #[error("Unclosed string - line {0}")]
    UnclosedString(usize),

    #[error(transparent)]
    IoError(#[from] std::io::Error),
}

pub type JuvinilResult<T, E = JuvinilError> = anyhow::Result<T, E>;
