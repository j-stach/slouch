
//! Simple OUCH v5.0 message library

/// Contains an OuchClient type for sending and receiving messages.
//pub mod client;
//pub use client::OuchClient;

/// Contains OuchIn and OuchOut types for protocol-compliant messages.
pub mod msg;

/// Contains strong types used in OUCH protocol messages.
pub mod types;

/// Contains an OuchError type for recoverable errors.
pub mod error;

// Contains unit tests for the library (minus client)
#[cfg(test)] mod test;

