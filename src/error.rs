#[derive(Debug, thiserror::Error)]
pub enum JuvinilError {
    #[error("Couldn't match token {0} to any defined expression - line {1}")]
    SyntaxError(String, usize),
    #[error("Unclosed string - line {0}")]
    UnclosedString(usize),
    #[error(transparent)]
    IoError(#[from] std::io::Error),
}

pub type JuvinilResult<T, E = JuvinilError> = anyhow::Result<T, E>;
