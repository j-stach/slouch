
mod enter;
mod cancel;
mod replace;
mod modify;
mod permission;
mod query;

pub use self::{ 
    enter::EnterOrder, 
    cancel::{ CancelOrder, MassCancel }, 
    replace::ReplaceOrder,
    modify::ModifyOrder,
    permission::{ EnableOrderEntry, DisableOrderEntry },
    query::QueryOrder,
};


use serde::{ Deserialize, Serialize };

#[derive(Debug, Serialize, Deserialize)]
pub enum OuchRequest {
    EnterOrder(EnterOrder),
    CancelOrder(CancelOrder),
    ReplaceOrder(ReplaceOrder),
    ModifyOrder(ModifyOrder),
    MassCancel(MassCancel),
    DisableOrderEntry(DisableOrderEntry),
    EnableOrderEntry(EnableOrderEntry),
    AccountQuery(AccountQuery),
}

impl OuchRequest {
    pub fn to_bytes(&self) -> Vec<u8> {
        match self {
            OuchRequest::EnterOrder(msg) => msg.encode(),
            OuchRequest::CancelOrder(msg) => msg.encode(),
            OuchRequest::ReplaceOrder(msg) => msg.encode(),
            OuchRequest::ModifyOrder(msg) => msg.encode(),
            OuchRequest::MassCancel(msg) => msg.encode(),
            OuchRequest::DisableOrderEntry(msg) => msg.encode(),
            OuchRequest::EnableOrderEntry(msg) => msg.encode(),
            OuchRequest::AccountQuery(msg) => msg.encode(),
        }
    }
}

