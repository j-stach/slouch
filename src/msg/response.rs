
mod accepted;
mod executed;
mod canceled;
mod rejected;
mod modified;
mod updated;
mod replaced;
mod permission;
mod broken;
mod event;
mod query;

pub use self::{
    accepted::OrderAccepted,
    executed::OrderExecuted,
    canceled::{ 
        OrderCanceled, 
        AiqCanceled, 
        CancelPending,
        CancelRejected,
        MassCancelResponse,
    },
    permission::{ 
        DisableOrderEntryResponse,
        EnableOrderEntryResponse,
    },
    modified::OrderModified,
    updated::{ 
        OrderPriorityUpdate, 
        OrderRestated 
    },
    rejected::OrderRejected,
    replaced::OrderReplaced,
    broken::BrokenTrade,
    event::SystemEvent,
    query::AccountQueryResponse
};

use std::convert::TryFrom;

use crate::error::OuchError;


///
#[derive(Debug, Clone)]
pub enum OuchResponse {
    OrderAccepted(OrderAccepted),
    OrderExecuted(OrderExecuted),
    OrderCanceled(OrderCanceled),
    OrderRejected(OrderRejected),
    OrderModified(OrderModified),
    OrderReplaced(OrderReplaced),
    AiqCanceled(AiqCanceled),
    CancelPending(CancelPending),
    CancelRejected(CancelRejected),
    MassCancelResponse(MassCancelResponse),
    DisableOrderEntryResponse(DisableOrderEntryResponse),
    EnableOrderEntryResponse(EnableOrderEntryResponse),
    OrderPriorityUpdate(OrderPriorityUpdate),
    OrderRestated(OrderRestated),
    BrokenTrade(BrokenTrade),
    SystemEvent(SystemEvent),
    AccountQueryResponse(AccountQueryResponse),
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

            b'A' => Ok(Self::OrderAccepted(OrderAccepted::parse(payload)?)),
            b'E' => Ok(Self::OrderExecuted(OrderExecuted::parse(payload)?)),
            b'C' => Ok(Self::OrderCanceled(OrderCanceled::parse(payload)?)),
            b'M' => Ok(Self::OrderModified(OrderModified::parse(payload)?)),
            b'J' => Ok(Self::OrderRejected(OrderRejected::parse(payload)?)),
            b'U' => Ok(Self::OrderReplaced(OrderReplaced::parse(payload)?)),
            b'D' => Ok(Self::AiqCanceled(AiqCanceled::parse(payload)?)),
            b'P' => Ok(Self::CancelPending(CancelPending::parse(payload)?)),
            b'I' => Ok(Self::CancelRejected(CancelRejected::parse(payload)?)),
            b'T' => Ok(Self::OrderPriorityUpdate(
                OrderPriorityUpdate::parse(payload)?
            )),

            b'R' => Ok(Self::OrderRestated(OrderRestated::parse(payload)?)),
            b'X' => Ok(Self::MassCancelResponse(
                MassCancelResponse::parse(payload)?
            )),

            b'G' => Ok(Self::DisableOrderEntryResponse(
                DisableOrderEntryResponse::parse(payload)?
            )),

            b'K' => Ok(Self::EnableOrderEntryResponse(
                EnableOrderEntryResponse::parse(payload)?
            )),

            b'B' => Ok(Self::BrokenTrade(BrokenTrade::parse(payload)?)),
            b'Q' => Ok(Self::AccountQueryResponse(
                    AccountQueryResponse::parse(payload)?
            )),

            b'S' => Ok(Self::SystemEvent(SystemEvent::parse(payload)?)),

            typ => Err(
                OuchError::UnknownResponse(typ as char, payload.to_vec())
            ),
        }
    }
}

