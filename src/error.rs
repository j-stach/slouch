
use thiserror::Error;

/// Errors that occur within this crate.
#[derive(Error, Debug)]
pub enum OuchError {

    #[error("Invalid message element: {0}")]
    InvalidElement(#[from] BadElementError),

    #[error("Unrecognized OUCH response type: {0}")]
    UnknownResponse(char, Vec<u8>),

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
#[derive(Error, Debug)]
pub enum BadElementError {

    #[error("Quantity {0} exceeds max shares allowed")]
    InvalidQuantity(u32),

    #[error("Invalid option tag for {0}")]
    InvalidOption(String),

    #[error("Invalid value for {0}")]
    InvalidType(#[from] nsdq_util::error::TypeError),
}


