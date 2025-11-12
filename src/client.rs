
use crate::{
    msg::{ OuchResponse, OuchRequest },
    error::OuchError,
    types::UserRefNum,
};

#[cfg(not(feature = "async"))] 
use std::{ 
    net::TcpStream,
    io::{ Read, Write },
};

#[cfg(feature = "async")]
use tokio::{ 
    net::TcpStream, 
    time::timeout as to,
    io::{ AsyncReadExt, AsyncWriteExt },
};

#[cfg(feature = "async")]
use std::time::Duration;


/// Convenience struct for coordinating order entry to an OUCH server.
pub struct OuchClient {
    stream: TcpStream,
    #[cfg(feature = "async")] timeout: Duration,
    buffer: Vec<u8>,
    next_user_ref_num: UserRefNum,
}

impl OuchClient {

    /// Use this method to GET the next valid UserRefNum.
    /// This method automatically increments the number to ensure uniqueness, 
    /// so only call this when you intend to use the value in a message.
    /// To PEEK at the value of the next UserRefNum without incrementing, 
    /// use the similarly-named method `next_user_ref_num`.
    pub fn new_user_ref_num(&mut self) -> UserRefNum {
        let new_num = self.next_user_ref_num.clone();
        self.next_user_ref_num.increment();
        new_num
    }

    /// Use this method to PEEK at the next UserRefNum.
    /// This method does NOT automatically increment to ensure uniqueness, 
    /// so do not use it to get a new UserRefNum to use in a message.
    /// To get the value of the next UserRefNum and increment it, 
    /// use the similarly-named method `new_user_ref_num`.
    pub fn next_user_ref_num(&self) -> &UserRefNum {
        &self.next_user_ref_num
    }

    /// Get mutable access to the TCP stream 
    /// (e.g., to configure it or to send a session management message).
    /// Treat this carefully so as to not disrupt communication.
    pub fn stream(&mut self) -> &mut TcpStream {
        &mut self.stream
    }

}

#[cfg(not(feature = "async"))]
impl OuchClient {

    /// Create an order entry Client by wrapping `std::net::TcpStream`.
    /// Assumes that the login request was accepted and the server is ready 
    /// to receive orders.
    pub fn wrap(stream: TcpStream) -> Result<Self, OuchError> {

        #[cfg(feature = "logs")] {
            log::info!("Creating new OuchClient instance...");
        }
        let mut client = OuchClient {
            stream,
            buffer: vec![0u8; 128],
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
                client.next_user_ref_num = aqr.next_user_ref_num();

                #[cfg(feature = "logs")] {
                    log::info!("Sync successful, new OuchClient connected.");
                }

                Ok(client)
            },
            
            _ => {

                let error = OuchError::UnexpectedResponse;

                #[cfg(feature = "logs")] {
                    log::error!("OuchClient encountered an error: {}", &error);
                }

                Err(error)
            },
        }

    }

    /// Send an OUCH order request to the server.
    pub fn send(&mut self, msg: OuchRequest) -> Result<(), OuchError> {

        #[cfg(feature = "logs")] {
            log::debug!("Sending {} request to server...", &msg);
        }

        let bytes = msg.encode();
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
        let data = &self.buffer[..n];

        #[cfg(feature = "logs")] {
            log::debug!("Response recieved.");
        }

        if let Ok((_data, response)) = OuchResponse::parse(data) {
            Ok(response)
        } else {
            // TODO: Extract information from IResult into custom error.
            Err(OuchError::Parse)
        }
    }
}

#[cfg(feature = "async")]
impl OuchClient {

    /// Create an async order entry Client by wrapping `tokio::net::TcpStream`.
    /// Assumes that the login request was accepted and the server is ready 
    /// to receive orders.
    pub async fn wrap(
        stream: TcpStream,
        timeout: Duration, 
    ) -> Result<Self, OuchError> {

        #[cfg(feature = "logs")] {
            log::info!("Creating new OuchClient instance...");
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
        client.send(crate::account_query!()).await?;
        let response = client.recv().await?;

        use OuchResponse::*;
        match response {

            AccountQueryResponse(aqr) => {
                client.next_user_ref_num = aqr.next_user_ref_num();

                #[cfg(feature = "logs")] {
                    log::info!("Sync successful, new OuchClient connected.");
                }

                Ok(client)
            },
            
            _ => {

                let error = OuchError::UnexpectedResponse;

                #[cfg(feature = "logs")] {
                    log::error!("OuchClient encountered an error: {}", &error);
                }

                Err(error)
            },
        }

    }

    /// Configure the TCP stream to time out after this duration.
    pub fn set_timeout(&mut self, duration: Duration) {
        self.timeout = duration
    }

    /// The TCP stream is configured to time out after this duration.
    pub fn timeout(&self) -> &Duration { &self.timeout }

    /// Send OUCH message to the server.
    pub async fn send(&mut self, msg: OuchRequest) -> Result<(), OuchError> {

        #[cfg(feature = "logs")] {
            log::debug!("Sending {} request to server...", &msg);
        }

        let bytes = msg.encode();
        to(self.timeout, self.stream.write_all(&bytes)).await??;

        #[cfg(feature = "logs")] {
            log::debug!("Request sent.");
        }

        Ok(())
    }

    /// Receive OUCH message from the server.
    pub async fn recv(&mut self) -> Result<OuchResponse, OuchError> {

        #[cfg(feature = "logs")] {
            log::debug!("Waiting for response from server...");
        }

        let n = to(self.timeout, self.stream.read(&mut self.buffer)).await??;
        let data = &self.buffer[..n];

        #[cfg(feature = "logs")] {
            log::debug!("Response recieved.");
        }

        if let Ok((_data, response)) = OuchResponse::parse(data) {
            Ok(response)
        } else {
            // TODO: Extract information from IResult into custom error.
            Err(OuchError::Parse)
        }
    }
}

