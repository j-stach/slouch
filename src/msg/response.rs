
mod event;
mod query;
mod updated;
mod broken;
mod permission;
mod executed;
mod canceled;
mod replaced;
mod rejected;
mod accepted;
mod modified;

pub use self::{
    event::SystemEvent,
    query::AccountQueryResponse,
    updated::{ 
        OrderPriorityUpdate, 
        OrderRestated 
    },
    broken::BrokenTrade,
    permission::{ 
        DisableOrderEntryResponse,
        EnableOrderEntryResponse,
    },
    executed::OrderExecuted,
    canceled::{ 
        OrderCanceled, 
        AiqCanceled, 
        CancelPending,
        CancelRejected,
        MassCancelResponse,
    },
    rejected::OrderRejected,
    replaced::OrderReplaced,
    accepted::OrderAccepted,
    modified::OrderModified,
};


macro_rules! ouch_responses {
    ($([$tag:expr] $msg_name:ident),*$(,)?) => {

        /// Server responses to expect in OUCH 5.0
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub enum OuchResponse {
            $(
                $msg_name($msg_name),
            )*
            /// May be a session management message or corrupted data.
            Unknown(char, Vec<u8>),
        }

        impl OuchResponse {

            /// 
            pub fn parse(input: &[u8]) -> nom::IResult<&[u8], Self> {
                
                let (input, msg_tag) = 
                    nom::bytes::complete::take(1usize)(input)?;

                match msg_tag[0] {
                    $(
                        $tag => {
                            let (input, msg) = $msg_name::parse(input)?;
                            Ok((input, Self::$msg_name(msg)))
                        },
                    )*

                    typ => Ok((
                        input, 
                        Self::Unknown(typ as char, input.to_vec())
                    )),
                }
            }
        }

    }
}

ouch_responses! {
    [b'S'] SystemEvent,
    [b'Q'] AccountQueryResponse,
    [b'T'] OrderPriorityUpdate,
    [b'R'] OrderRestated,
    [b'B'] BrokenTrade,
    [b'G'] DisableOrderEntryResponse,
    [b'K'] EnableOrderEntryResponse,
    [b'E'] OrderExecuted,
    [b'A'] OrderAccepted,
    [b'M'] OrderModified,
    [b'C'] OrderCanceled,
    [b'J'] OrderRejected,
    [b'U'] OrderReplaced,
    [b'D'] AiqCanceled,
    [b'P'] CancelPending,
    [b'I'] CancelRejected,
    [b'X'] MassCancelResponse,
}


