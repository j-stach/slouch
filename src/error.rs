
use thiserror::Error;

/// Errors that occur within this crate.
#[derive(Error, Debug)]
pub enum OuchError {

    #[error("Message element is not protocol compliant: {0}")]
    InvalidElement(#[from] BadElementError),

    #[error("Failed to parse: {0}")]
    Parse(String),

    #[error("OuchClient suffered an IO error: {0}")]
    ClientIo(#[from] std::io::Error),
}

// ClientError
// MessageError
// InvalidElementError

/// Errors that occur from invalid message elements.
#[derive(Error, Debug)]
pub enum BadElementError {

    #[error("Invalid OrderToken: {0}")]
    OrderToken(String),

    #[error("Invalid FirmId: {0}")]
    FirmId(String),

    #[error("Invalid order token prefix: {0}")]
    TokenPrefix(String),

    #[error("Invalid StockSymbol: {0}")]
    StockSymbol(String),
}


