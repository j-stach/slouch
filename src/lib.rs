
//! Simple OUCH v5.0 message library

/// Contains OuchIn and OuchOut types for protocol-compliant messages.
pub mod msg;

/// Contains an OuchClient type for sending and receiving messages.
pub mod client;

/// Contains an OuchError type for recoverable errors.
pub mod error;

/// Contains strong types used in OUCH protocol messages.
pub mod types;

/// Helper functions used throughout the crate.
pub mod helper;

