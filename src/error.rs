#[derive(Debug, thiserror::Error)]
pub enum JuvinilError {
    #[error("Couldn't match token {0} to any defined expression - line {1}")]
    LexicalError(String, usize),
    #[error("Syntax error")]
    SyntaxError,
    #[error("Parsing error")]
    ParsingError,
    #[error("Unclosed string - line {0}")]
    UnclosedString(usize),
    #[error(transparent)]
    IoError(#[from] std::io::Error),
}

pub type JuvinilResult<T, E = JuvinilError> = anyhow::Result<T, E>;
