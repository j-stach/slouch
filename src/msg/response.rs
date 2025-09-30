
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

            b'A' => Ok(OrderAccepted::parse(payload)?
                .map(OuchResponse::OrderAccepted)),

            b'E' => Ok(OrderExecuted::parse(payload)?
                .map(OuchResponse::OrderExecuted)),

            b'C' => Ok(OrderCanceled::parse(payload)?
                .map(OuchResponse::OrderCanceled)),

            b'M' => Ok(OrderModified::parse(payload)?
                .map(OuchResponse::OrderModified)),

            b'J' => Ok(OrderRejected::parse(payload)?
                .map(OuchResponse::OrderRejected)),

            b'U' => Ok(OrderReplaced::parse(payload)?
                .map(OuchResponse::OrderReplaced)),

            b'D' => Ok(AiqCanceled::parse(payload)?
                .map(OuchResponse::AiqCanceled)),

            b'P' => Ok(CancelPending::parse(payload)?
                .map(OuchResponse::CancelPending)),

            b'I' => Ok(CancelRejected::parse(payload)?
                .map(OuchResponse::CancelRejected)),

            b'T' => Ok(OrderPriorityUpdate::parse(payload)?
                .map(OuchResponse::OrderPriorityUpdate)),

            b'R' => Ok(OrderRestated::parse(payload)?
                .map(OuchResponse::OrderRestated)),

            b'X' => Ok(MassCancelResponse::parse(payload)?
                .map(OuchResponse::MassCancelResponse)),

            b'G' => Ok(DisableOrderEntryResponse::parse(payload)?
                .map(OuchResponse::DisableOrderEntryResponse)),

            b'K' => Ok(EnableOrderEntryResponse::parse(payload)?
                .map(OuchResponse::EnableOrderEntryResponse)),

            b'B' => Ok(BrokenTrade::parse(payload)?
                .map(OuchResponse::BrokenTrade)),

            b'Q' => Ok(AccountQueryResponse::parse(payload)?
                .map(OuchResponse::AccountQueryResponse)),

            b'S' => Ok(SystemEvent::parse(payload)?
                .map(OuchResponse::SystemEvent)),

            typ => Err(OuchError::UnknownResponse(typ, payload)),
        }
    }
}

