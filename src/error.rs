use crate::lexical_analysis::token::{Token, TokenType};

//Implementation of all the different errors that can
//occur during all steps of the compiler
#[derive(Debug, thiserror::Error)]
pub enum JuvinilError {
    #[error("Couldn't match token {0} to any defined expression - line {1}")]
    LexicalError(String, usize),

    #[error("Syntax error - Expecting <{0:?} {1}>, found {2:?} - line {3}")]
    SyntaxError(TokenType, String, Token, usize),

    #[error("Duplicate Variable - Variable `{0}` was already declared - line {1}")]
    DuplicateVariable(String, usize),

    #[error("Undeclared Variable - Variable `{0}` was not declared - line {1}")]
    UndeclaredVariable(String, usize),

    #[error("Unassigned Variable - Variable `{0}` was not assigned before being used - line {1}")]
    UnassignedVariable(String, usize),

    #[error("Duplicate Function - Function `{0}` was already declared - line {1}")]
    DuplicateFunction(String, usize),

    #[error("Undeclared Function - Function `{0}` was not declared - line {1}")]
    UndeclaredFunction(String, usize),

    #[error("Unclosed string - line {0}")]
    UnclosedString(usize),

    #[error(transparent)]
    IoError(#[from] std::io::Error),
}

pub type JuvinilResult<T, E = JuvinilError> = anyhow::Result<T, E>;
