//! Crate-wide error type.
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DevkitError {
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),

    #[error("parse error: {0}")]
    Parse(String),

    #[error("validation error: {0}")]
    Validation(String),
}
