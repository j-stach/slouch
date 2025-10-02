
use thiserror::Error;

/// Errors that occur within this crate.
#[derive(Error, Debug)]
pub enum OuchError {

    #[error("Invalid message element: {0}")]
    InvalidElement(#[from] BadElementError),

    #[error("Unrecognized OUCH response type: {0}")]
    UnknownResponse(char, Vec<u8>),

    #[error("Insufficient data to parse: {0}")]
    Parse(String),

    /// This only occurs in the client.
    #[error("OuchClient suffered an IO error: {0}")]
    ClientIo(#[from] std::io::Error),

    /// This only occurs in the client.
    #[error("OuchClient encountered an unexpected response to initial Query")]
    UnexpectedResponse,

    #[cfg(feature = "async")]
    /// This only occurs in the client.
    #[error("OuchClient timed out: {0}")]
    AsyncTimeout(#[from] tokio::time::error::Elapsed),

}

/// Errors that occur from invalid message elements.
#[derive(Error, Debug, PartialEq, Eq)]
pub enum BadElementError {

    #[error("{0} should be {1} bytes, found {2}")]
    WrongSize(String, usize, usize),

    #[error("{0} is not uppercase alphabetic ASCII")]
    NotUppercaseAlpha(String),

    #[error("{0} is not alphabetic ASCII")]
    NotAlpha(String),

    #[error("Invalid string: {0}")]
    InvalidAscii(#[from] std::str::Utf8Error),

    #[error("{0} is an invalid option for {1}")]
    InvalidEnum(String, String),

    #[error("Invalid value for {0}")]
    InvalidValue(String),

    #[error("Invalid option tag for {0}")]
    InvalidOption(String),
}


