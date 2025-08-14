
use std::io::{ Read, Write };
use std::net::TcpStream;
use std::time::Duration;

use crate::{
    msg::{ OuchOut, OuchIn },
    types::{ OrderToken, FirmId, StockSymbol },
    error::OuchError
};

/// Convenience struct for coordinating communication with OUCH server.
pub struct OuchClient {
    stream: TcpStream,
    buffer: Vec<u8>,
    timeout: Duration,
    firm_id: FirmId,
    order_token_prefix: String,
    order_token_counter: u64,
}

impl OuchClient {

    /// Create a Client connected to the provided TCP address.
    pub fn connect(
        // TODO: Use SocketAddr type
        addr: &str, 
        timeout: Duration, 
        firm_id: FirmId, 
        // TODO: Does this need length checked?
        order_token_prefix: String
    ) -> Result<Self, OuchError> {

        let stream = TcpStream::connect(addr)?;
        stream.set_read_timeout(Some(timeout))?;
        stream.set_write_timeout(Some(timeout))?;

        Ok(OuchClient {
            stream,
            buffer: vec![0u8; 128],
            timeout,
            firm_id,
            order_token_prefix,
            order_token_counter: 0,
        })
    }

    // TODO: Use TokenGenerator
    /*
    fn next_order_token(&mut self) -> Result<OrderToken, OuchError> {
        self.order_token_counter += 1;
        OrderToken::from_parts(
            &self.order_token_prefix, 
            self.order_token_counter
        )
    }
    */

    /// The TCP stream is configured to time out after this duration.
    pub fn timeout(&self) -> &Duration { &self.timeout }

    /// The client is configured to use this firm ID for placing orders.
    pub fn firm_id(&self) -> &FirmId { &self.firm_id }

    /// The client uses this prefix for generating order tokens.
    pub fn order_token_prefix(&self) -> &str { &self.order_token_prefix }

    /// Send OUCH message to the server.
    pub fn send(&mut self, msg: OuchOut) -> Result<(), OuchError> {

        let bytes = msg.to_bytes();
        self.stream.write_all(&bytes)?;
        Ok(())
    }

    /// Receive OUCH message from the server.
    pub fn recv(&mut self) -> Result<OuchIn, OuchError> {

        let n = self.stream.read(&mut self.buffer)?;
        OuchIn::try_from(&self.buffer[..n])
            .map_err(|e| {
                std::io::Error::new(std::io::ErrorKind::InvalidData, e).into()
            })
    }
}
