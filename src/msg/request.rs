
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
    query::AccountQuery,
};


#[derive(Debug, Clone)]
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


/// Create an AccountQuery message.
#[macro_export]
macro_rules! account_query {
    () => {
        crate::msg::OuchRequest::AccountQuery(crate::msg::AccountQuery::new())
    }
}

/// Create a DisableOrderEntry request message.
#[macro_export]
macro_rules! disable_entry {
    (user_ref_num: $f1:expr, firm: $f2:expr $(,)?) => {
        crate::msg::OuchRequest::DisableOrderEntry(
            crate::msg::DisableOrderEntry::new($f1, $f2)
        )
    }
}

/// Create an EnableOrderEntry request message.
#[macro_export]
macro_rules! enable_entry {
    (user_ref_num: $f1:expr, firm: $f2:expr $(,)?) => {
        crate::msg::OuchRequest::EnableOrderEntry(
            crate::msg::EnableOrderEntry::new($f1, $f2)
        )
    }
}

/// Create a MassCancel request message.
#[macro_export]
macro_rules! mass_cancel {
    (user_ref_num: $f1:expr, firm: $f2:expr, symbol: $f3:expr $(,)?) => {
        crate::msg::OuchRequest::MassCancel(
            crate::msg::MassCancel::new($f1, $f2, $f3)
        )
    }
}

/// Create a CancelOrder request message.
#[macro_export]
macro_rules! cancel {
    (user_ref_num: $f1:expr, quantity: $f2:expr $(,)?) => {
        crate::msg::OuchRequest::CancelOrder(
            crate::msg::CancelOrder::new($f1, $f2)
        )
    }
}

/// Create an EnterOrder request message.
#[macro_export]
macro_rules! enter {
    (
        user_ref_num: $f1:expr,
        side: $f2:expr,
        quantity: $f3:expr,
        symbol: $f4:expr,
        price: $f5:expr,
        time_in_force: $f6:expr,
        display: $f7:expr,
        capacity: $f8:expr,
        intermarket_sweep_eligibility: $f9:expr,
        cross_type: $f10:expr,
        order_token: $f11:expr $(,)?
    ) => {
        crate::msg::OuchRequest::EnterOrder(
            crate::msg::EnterOrder::new(
                $f1, $f2, $f3, $f4, $f5, $f6, $f7, $f8, $f9, $f10, $f11
            )
        )
    };
}

/// Create a ReplaceOrder request message.
#[macro_export]
macro_rules! replace {
    (
        user_ref_num: $f1:expr,
        quantity: $f2:expr,
        price: $f3:expr,
        time_in_force: $f4:expr,
        display: $f5:expr,
        intermarket_sweep_eligibility: $f6:expr,
        order_token: $f7:expr $(,)?
    ) => {
        crate::msg::OuchRequest::ReplaceOrder(
            crate::msg::ReplaceOrder::new($f1, $f2, $f3, $f4, $f5, $f6, $f7)
        )
    };
}

/// Create a ModifyOrder request message.
#[macro_export]
macro_rules! modify {
    (user_ref_num: $f1:expr, side: $f2:expr, quantity: $f3:expr$(,)?) => {
        crate::msg::OuchRequest::ModifyOrder(
            crate::msg::ModifyOrder::new($f1, $f2, $f3)
        )
    };
}

