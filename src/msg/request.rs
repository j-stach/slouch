
mod cancel;
mod enter;
mod permission;
mod query;
mod modify;
mod replace;

pub use self::{
    cancel::{ CancelOrder, MassCancel },
    enter::EnterOrder,
    permission::{ EnableOrderEntry, DisableOrderEntry },
    query::AccountQuery,
    modify::ModifyOrder,
    replace::ReplaceOrder,
};

macro_rules! ouch_requests {
    ($([$tag:expr] $msg_kind:ident),*$(,)?) => {

        /// Client requests that can be sent via OUCH.
        /// All requests may be repeated benignly. 
        /// This gives the client the ability to re-send any request if it is 
        /// uncertain whether NASDAQ received it in the case of a connection 
        /// loss or an application error.
        ///
        /// All inbound messages on an OUCH port are processed sequentially. 
        /// This guarantees that if two orders are entered consecutively on 
        /// the same connection, the first order entered will always be 
        /// accepted first.
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub enum OuchRequest {
            $(
                $msg_kind($msg_kind),
            )*
        }

        impl OuchRequest {

            /// Encode message to bytes.
            pub fn encode(&self) -> Vec<u8> {

                match self {
                    $(
                        Self::$msg_kind(msg) => {
                            let mut bytes = vec![$tag];
                            bytes.extend(msg.encode());
                            bytes
                        },
                    )*
                }
            }

            /// Get the options attached to this request.
            pub fn options(&self) -> &Vec<$crate::msg::options::TagValue> {
                match self {
                    $(
                        Self::$msg_kind(msg) => msg.options(),
                    )*
                }
            }

            /// Add a new optional field to the request's appendage.
            /// If a `TagValue` of the same variant already exists,
            /// it will be overwritten in place by the new optional field.
            pub fn add_option(
                &mut self, 
                option: $crate::msg::options::TagValue
            ) -> Result<(), $crate::error::OuchError> {
                match self {
                    $(
                        Self::$msg_kind(msg) => Ok(msg.add_option(option)?),
                    )*
                }
            }

        }

        impl std::fmt::Display for OuchRequest {
            /// Write the name of the request variant only (no data).
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let ch = match self {
                    $(
                        Self::$msg_kind(..) => $tag as char,
                    )*
                };

                ch.fmt(f)
            }
        }
        
    }
}

ouch_requests!{
    [b'Q'] AccountQuery,
    [b'D'] DisableOrderEntry,
    [b'E'] EnableOrderEntry,
    [b'C'] MassCancel,
    [b'X'] CancelOrder,
    [b'O'] EnterOrder,
    [b'U'] ReplaceOrder,
    [b'M'] ModifyOrder,
}

