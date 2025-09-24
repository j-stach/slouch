

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
    AiqStrategy,
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

    // Data contains package without type tag, 
    // so all offsets should be one less than those in the official spec.
    pub(super) fn parse(data: &[u8]) -> Result<Self, OuchError> {

        if data.len() < 19 {
            return Err(OuchError::Parse("OrderCanceled".to_string()))
        }

        Ok(Self {
            timestamp: {
                let ts = u64_from_be_bytes(&data[0..=7])?;
                nanosec_from_midnight(ts)
            },
            user_ref_num: UserRefNum::parse(&data[8..=11])?,
            quantity: u32_from_be_bytes(&data[12..=15])?, 
            reason: CancelReason::parse(data[16])?;
            optional_appendage: OptionalAppendage::parse(&data[17..])?
        })
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

    // Data contains package without type tag, 
    // so all offsets should be one less than those in the official spec.
    pub(super) fn parse(data: &[u8]) -> Result<Self, OuchError> {

        if data.len() < 33 {
            return Err(OuchError::Parse("AiqCanceled".to_string()))
        }

        Ok(Self {
            timestamp: {
                let ts = u64_from_be_bytes(&data[0..=7])?;
                nanosec_from_midnight(ts)
            },
            user_ref_num: UserRefNum::parse(&data[8..=11])?,
            decrement_shares: u32_from_be_bytes(&data[12..=15])?, 
            reason: CancelReason::parse(data[16])?,
            quantity_prevented: u32_from_be_bytes(&data[17..=20])?, 
            execution_price: Price::parse(&data[21..=28])?,
            liquidity_flag: LiquidityFlag::parse(&data[29])?,
            aiq_strategy: AiqStrategy::parse(&data[30])?,
            optional_appendage: OptionalAppendage::parse(&data[31..])?
        })
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

    // Data contains package without type tag, 
    // so all offsets should be one less than those in the official spec.
    pub(super) fn parse(data: &[u8]) -> Result<Self, OuchError> {

        if data.len() < 14 {
            return Err(OuchError::Parse("CancelPending".to_string()))
        }

        Ok(Self {
            timestamp: {
                let ts = u64_from_be_bytes(&data[0..=7])?;
                nanosec_from_midnight(ts)
            },
            user_ref_num: UserRefNum::parse(&data[8..=11])?,
            optional_appendage: OptionalAppendage::parse(&data[12..])?
        })
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

    // Data contains package without type tag, 
    // so all offsets should be one less than those in the official spec.
    pub(super) fn parse(data: &[u8]) -> Result<Self, OuchError> {

        if data.len() < 14 {
            return Err(OuchError::Parse("CancelRejected".to_string()))
        }

        Ok(Self {
            timestamp: {
                let ts = u64_from_be_bytes(&data[0..=7])?;
                nanosec_from_midnight(ts)
            },
            user_ref_num: UserRefNum::parse(&data[8..=11])?,
            optional_appendage: OptionalAppendage::parse(&data[12..])?
        })
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

    // Data contains package without type tag, 
    // so all offsets should be one less than those in the official spec.
    pub(super) fn parse(data: &[u8]) -> Result<Self, OuchError> {

        if data.len() < 26 {
            return Err(OuchError::Parse("MassCancelResponse".to_string()))
        }

        Ok(Self {
            timestamp: {
                let ts = u64_from_be_bytes(&data[0..=7])?;
                nanosec_from_midnight(ts)
            },
            user_ref_num: UserRefNum::parse(&data[8..=11])?,
            firm: FirmId::parse(&data[12..=15])?,
            symbol: StockSymbol::parse(&data[16..=23])?,
            optional_appendage: OptionalAppendage::parse(&data[24..])?
        })
    }

}


