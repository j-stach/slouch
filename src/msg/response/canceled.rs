

use chrono::NaiveTime;
use serde::{ Deserialize, Serialize };

use crate::{
    error::OuchError,
    helper::{ 
        u32_from_be_bytes, 
        u64_from_be_bytes,
        nanosec_from_midnight
    }
};

use crate::types::{ 
    UserRefNum,
    Side,
    StockSymbol, 
    Price,
    TimeInForce,
    Display,
    Capacity,
    CrossType,
    OrderState,
    CancelReason,
    OrderToken
};


///
#[derive(Debug, Serialize, Deserialize)]
pub struct OrderCanceled {
    pub timestamp: NaiveTime,
    pub user_ref_num: UserRefNum,
    pub quantity: u32,
    pub reason: CancelReason,
    optional_appendage: OptionalAppendage
}

impl OrderCanceled {

    pub(super) fn parse(data: &[u8]) -> Result<Self, OuchError> {

        todo!{}
    }

}


///
#[derive(Debug, Serialize, Deserialize)]
pub struct AiqCanceled {
    pub timestamp: NaiveTime,
    pub user_ref_num: UserRefNum,
    /// The number of shares just decremented from the order. 
    /// This number is incremental, not cumulative.
    pub decrement_shares: u32,
    /// For an AIQ Cancel message, this value will always be "Q".
    pub reason: CancelReason,
    /// Shares that would have executed if the trade had occurred.
    pub quantity_prevented: u32,
    /// Price at which the trade would have occurred.
    pub execution_price: Price,
    /// Liquidity flag the order would have received.
    pub liquidity_flag: LiquidityFlag,
    /// AIQ strategy used on the cancelled order.
    pub aiq_strategy: AiqStrategy,
    optional_appendage: OptionalAppendage
}

impl AiqCanceled {

    pub(super) fn parse(data: &[u8]) -> Result<Self, OuchError> {

        todo!{}
    }

}


///
#[derive(Debug, Serialize, Deserialize)]
pub struct CancelPending {
    pub timestamp: NaiveTime,
    pub user_ref_num: UserRefNum,
    optional_appendage: OptionalAppendage
}

impl CancelPending {

    pub(super) fn parse(data: &[u8]) -> Result<Self, OuchError> {

        todo!{}
    }

}


///
#[derive(Debug, Serialize, Deserialize)]
pub struct CancelRejected {
    pub timestamp: NaiveTime,
    pub user_ref_num: UserRefNum,
    optional_appendage: OptionalAppendage
}

impl CancelRejected {

    pub(super) fn parse(data: &[u8]) -> Result<Self, OuchError> {

        todo!{}
    }

}


///
#[derive(Debug, Serialize, Deserialize)]
pub struct MassCancelResponse {
    pub timestamp: NaiveTime,
    pub user_ref_num: UserRefNum,
    pub firm: FirmId,
    pub symbol: StockSymbol,
    optional_appendage: OptionalAppendage
}

impl MassCancelResponse {

    pub(super) fn parse(data: &[u8]) -> Result<Self, OuchError> {

        todo!{}
    }

}


