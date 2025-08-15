
mod token_generator;
mod token_prefix;

pub use token_prefix::TokenPrefix;
pub use token_generator::TokenGenerator;


use std::io::{ Read, Write, Error, ErrorKind };
use std::net::{ TcpStream, SocketAddr };
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
    // TODO: Will be changed to trait obj for flexibility
    order_token_generator: TokenGenerator,
}

impl OuchClient {

    /// Create a Client connected to the provided TCP address.
    pub fn connect(
        addr: SocketAddr, 
        timeout: Duration, 
        firm_id: FirmId, 
        order_token_prefix: &str
    ) -> Result<Self, OuchError> {

        let stream = TcpStream::connect(addr)?;
        stream.set_read_timeout(Some(timeout))?;
        stream.set_write_timeout(Some(timeout))?;

        Ok(OuchClient {
            stream,
            buffer: vec![0u8; 128],
            timeout,
            firm_id,
            order_token_generator: TokenGenerator::new(order_token_prefix)?,
        })
    }

    /// The TCP stream is configured to time out after this duration.
    pub fn timeout(&self) -> &Duration { &self.timeout }


    /// Configure the TCP stream to time out after this duration.
    pub fn set_timeout(&mut self, duration: Duration) -> Result<(), OuchError> { 

        self.stream.set_read_timeout(Some(duration))?;
        self.stream.set_write_timeout(Some(duration))?;
        Ok(self.timeout = duration)
    }

    /// The client is configured to use this firm ID for placing orders.
    pub fn firm_id(&self) -> &FirmId { &self.firm_id }

    /// The client uses this struct to generate order tokens.
    /// NOTE: Returns a mutable reference, so be careful what you do with it.
    pub fn token_generator(&mut self) -> &mut TokenGenerator { 
        &mut self.order_token_generator 
    }

}

impl OuchClient {

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
            .map_err(|e| Error::new(ErrorKind::InvalidData, e).into())
    }
}
