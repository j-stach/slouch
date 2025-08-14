
use thiserror::Error;

/// Errors that occur within this crate.
#[derive(Error, Debug)]
pub enum OuchError {

    #[error("Invalid message element: {0}")]
    InvalidElement(#[from] BadElementError),

    #[error("Insufficient data to parse: {0}")]
    Parse(String),

    #[error("OuchClient suffered an IO error: {0}")]
    ClientIo(#[from] std::io::Error),
}

/// Errors that occur from invalid message elements.
#[derive(Error, Debug)]
pub enum BadElementError {

    #[error("{0} should be {1} bytes, found {2}")]
    WrongSize(String, usize, usize),

    #[error("{0} is not valid ASCII")]
    NotAscii(String),
}


