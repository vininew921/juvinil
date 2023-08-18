#[derive(Debug, thiserror::Error)]
pub enum JuvinilError {
    #[error("Couldn't match token {0} to any defined expression - line {1}")]
    NoRegexMatch(String, usize),
}

pub type JuvinilResult<T, E = JuvinilError> = anyhow::Result<T, E>;
