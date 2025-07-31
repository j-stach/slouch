
use thiserror::Error;

/// Errors that occur within this crate.
#[derive(Error, Debug)]
pub enum OuchError {

    #[error("Invalid order token: {0}")]
    InvalidOrderToken(String),

    #[error("Parse error: {0}")]
    Parse(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

