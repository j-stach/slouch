
use thiserror::Error;

/// Errors that occur within this crate.
#[derive(Error, Debug)]
pub enum OuchError {

    #[error("Invalid message element: {0}")]
    InvalidElement(#[from] BadElementError),

    #[error("Unrecognized OUCH response type: {0}, data: {1}")]
    UnknownResponse(char, Vec<u8>),

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

    #[error("{0} is not alphanumeric ASCII (or space)")]
    NotValidAscii(String),

    #[error("{0} is not uppercase alphabetic ASCII")]
    NotUppercaseAlpha(String),

    #[error("{0} is not alphabetic ASCII (or space)")]
    NotAlpha(String),

    #[error("{0} is an invalid option for {1}")]
    InvalidEnum(char, String),

    #[error("Invalid value for {0}")]
    InvalidValue(String),

    #[error("Invalid option tag for {0}")]
    InvalidOption(String),

}


