
mod accepted;
mod executed;
mod canceled;
mod rejected;
mod broken;
mod event;

pub use self::{
    accepted::OrderAccepted,
    executed::OrderExecuted,
    canceled::OrderCanceled,
    rejected::OrderRejected,
    broken::BrokenTrade,
    event::SystemEvent
};

use std::convert::TryFrom;

use crate::error::OuchError;


#[derive(Debug)]
pub enum OuchResponse {
    OrderAccepted(OrderAccepted),
    OrderExecuted(OrderExecuted),
    OrderCanceled(OrderCanceled),
    OrderRejected(OrderRejected),
    BrokenTrade(BrokenTrade),
    SystemEvent(SystemEvent),
    Unknown(u8, Vec<u8>),
}

impl TryFrom<&[u8]> for OuchResponse {
    type Error = OuchError;

    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {

        if data.is_empty() {
            return Err(OuchError::Parse("Empty message".to_string()))
        }

        let msg_type = data[0];
        let payload = &data[1..];

        match msg_type {
            b'A' => OrderAccepted::parse(payload).map(OuchIn::OrderAccepted),
            b'E' => OrderExecuted::parse(payload).map(OuchIn::OrderExecuted),
            b'C' => OrderCanceled::parse(payload).map(OuchIn::OrderCanceled),
            b'R' => OrderRejected::parse(payload).map(OuchIn::OrderRejected),
            b'B' => BrokenTrade::parse(payload).map(OuchIn::BrokenTrade),
            b'S' => SystemEvent::parse(payload).map(OuchIn::SystemEvent),
            _ => Ok(OuchIn::Unknown(msg_type, payload.to_vec())),
        }
    }
}

