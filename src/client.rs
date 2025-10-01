
use std::io::{ Read, Write };
use std::net::{ TcpStream, SocketAddr };
use std::time::Duration;

use crate::{
    msg::{ OuchResponse, OuchRequest },
    error::OuchError,
    types::UserRefNum,
};


/// Convenience struct for coordinating communication with OUCH server.
pub struct OuchClient {
    stream: TcpStream,
    buffer: Vec<u8>,
    timeout: Duration,
    next_user_ref_num: UserRefNum,
}

impl OuchClient {

    /// Create a Client connected to the provided TCP address.
    pub fn connect(
        addr: SocketAddr, 
        timeout: Duration, 
    ) -> Result<Self, OuchError> {

        #[cfg(feature = "logs")] {
            log::info!("Creating new OuchClient instance...");
            log::debug!("Connecting to {}", &addr);
        }

        let stream = TcpStream::connect(addr)?;

        #[cfg(feature = "logs")] {
            log::debug!("Setting stream R/W timeout...");
        }

        stream.set_read_timeout(Some(timeout))?;
        stream.set_write_timeout(Some(timeout))?;

        #[cfg(feature = "logs")] {
            log::debug!("Stream timeout set.");
        }

        let mut client = OuchClient {
            stream,
            buffer: vec![0u8; 128],
            timeout,
            next_user_ref_num: UserRefNum::new(),
        };

        #[cfg(feature = "logs")] {
            log::info!("Syncing account with server...");
        }

        // Send AccountQuery to verify connection & get next UserRefNum
        client.send(crate::account_query!())?;
        let response = client.recv()?;

        use OuchResponse::*;
        match response {

            AccountQueryResponse(aqr) => {
                client.next_user_ref_num = aqr.next_user_ref_num().clone();

                #[cfg(feature = "logs")] {
                    log::info!("Sync successful, new OuchClient connected.");
                }

                Ok(client)
            },
            
            // TBD: What if you have no account, or connection is rejected?
            // Will it simply timeout?
            _ => {

                let error = OuchError::UnexpectedResponse;

                #[cfg(feature = "logs")] {
                    log::error!("OuchClient encountered an error: {}", &error);
                }

                Err(error)
            },
        }

    }

    /// The TCP stream is configured to time out after this duration.
    pub fn timeout(&self) -> &Duration { &self.timeout }


    /// Configure the TCP stream to time out after this duration.
    pub fn set_timeout(&mut self, duration: Duration) -> Result<(), OuchError> {

        #[cfg(feature = "logs")] {
            log::debug!("Setting stream R/W timeout...");
        }

        self.stream.set_read_timeout(Some(duration))?;
        self.stream.set_write_timeout(Some(duration))?;

        #[cfg(feature = "logs")] {
            log::debug!("Stream timeout set.");
        }

        Ok(self.timeout = duration)
    }

    /// Use this function to get the next valid UserRefNum.
    /// Automatically increments, so only call this when you intend 
    /// to use the value in a message.
    /// To peek at the value of the next UserRefNum without incrementing, 
    /// use the similarly-named `next_user_ref_num`.
    pub fn new_user_ref_num(&mut self) -> UserRefNum {
        let new_num = self.next_user_ref_num.clone();
        self.next_user_ref_num.increment();
        new_num
    }

    /// Use this method to "peek" at the next UserRefNum.
    /// Does not increment, so do not use this method to get a UserRefNum
    /// to use in a message.
    /// To get the value of the next UserRefNum and increment it, 
    /// use the similarly-named `new_user_ref_num`.
    pub fn next_user_ref_num(&self) -> &UserRefNum {
        &self.next_user_ref_num
    }
}

impl OuchClient {

    /// Send OUCH message to the server.
    pub fn send(&mut self, msg: OuchRequest) -> Result<(), OuchError> {

        #[cfg(feature = "logs")] {
            log::debug!("Sending {} request to server...", &msg);
        }

        let bytes = msg.to_bytes();
        self.stream.write_all(&bytes)?;

        #[cfg(feature = "logs")] {
            log::debug!("Request sent.");
        }

        Ok(())
    }

    /// Receive OUCH message from the server.
    pub fn recv(&mut self) -> Result<OuchResponse, OuchError> {

        #[cfg(feature = "logs")] {
            log::debug!("Waiting for response from server...");
        }

        let n = self.stream.read(&mut self.buffer)?;

        #[cfg(feature = "logs")] {
            log::debug!("Response recieved.");
        }

        OuchResponse::try_from(&self.buffer[..n])
    }
}

