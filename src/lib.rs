
//! Simple OUCH-protocol message library

/// Contains OuchIn and OuchOut types for protocol-compliant messages.
pub mod msg;

/// Contains a dedicated OrderToken type that ensures valid tokens.
pub mod token;

/// Contains an OuchClient type for sending and receiving messages.
pub mod client;

/// Contains an OuchError type for recoverable errors.
pub mod error;

// Helper functions used throughout the crate.
mod helper;

