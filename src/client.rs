
use std::io::{ Read, Write };
use std::net::TcpStream;
use std::time::Duration;

use crate::{
    msg::{ OuchOut, OuchIn },
    token::OrderToken,
    trade::{ FirmId, StockSymbol },
};

///
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
        order_token_prefix: String
    ) -> std::io::Result<Self> {

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

    fn next_order_token(&mut self) -> String {
        self.order_token_counter += 1;
        // TODO OrderToken from_parts
        format!("{}{:08}", self.order_token_prefix, self.order_token_counter)
    }

    ///
    pub fn send(&mut self, msg: OuchOut) -> std::io::Result<()> {

        let bytes = msg.to_bytes();
        self.stream.write_all(&bytes)
    }

    ///
    pub fn recv(&mut self) -> std::io::Result<OuchIn> {

        let n = self.stream.read(&mut self.buffer)?;
        OuchIn::try_from(&self.buffer[..n])
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))
    }
}
