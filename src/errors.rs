use regex;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParseLLMSError {
    #[error("Invalid LLM txt spec format")]
    InvalidSpec,
    #[error("Regex error: {0}")]
    RegexError(#[from] regex::Error),
}
