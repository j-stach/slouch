
use chrono::NaiveTime;

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
    StockSymbol, 
    Price,
    CancelReason,
    AiqStrategy,
    FirmId,
    LiquidityFlag
};

use crate::msg::options::*;


///
#[derive(Debug, Clone)]
pub struct OrderCanceled {
    timestamp: NaiveTime,
    user_ref_num: UserRefNum,
    quantity: u32,
    reason: CancelReason,
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
            reason: CancelReason::parse(data[16])?,
            optional_appendage: OptionalAppendage::parse(&data[17..])?
        })
    }
    
    pub fn timestamp(&self) -> &NaiveTime { &self.timestamp }

    /// Gets the user reference number.
    pub fn user_ref_num(&self) -> UserRefNum { self.user_ref_num }

    pub fn quantity(&self) -> u32 { self.quantity }
    
    pub fn reason(&self) -> CancelReason { self.reason }
    
    /// Get read-only access to the message's optional fields.
    pub fn options(&self) -> &Vec<TagValue> {
       &self.optional_appendage.tag_values()
    }

}


///
#[derive(Debug, Clone)]
pub struct AiqCanceled {
    timestamp: NaiveTime,
    user_ref_num: UserRefNum,
    decrement_shares: u32,
    reason: CancelReason,
    quantity_prevented: u32,
    execution_price: Price,
    liquidity_flag: LiquidityFlag,
    aiq_strategy: AiqStrategy,
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
            liquidity_flag: LiquidityFlag::parse(data[29])?,
            aiq_strategy: AiqStrategy::parse(data[30])?,
            optional_appendage: OptionalAppendage::parse(&data[31..])?
        })
    }
    
    pub fn timestamp(&self) -> NaiveTime { self.timestamp }

    /// Gets the user reference number.
    pub fn user_ref_num(&self) -> UserRefNum { self.user_ref_num }

    /// Liquidity flag the order would have received.
    pub fn liquidity_flag(&self) -> LiquidityFlag { self.liquidity_flag }

    /// Price at which the trade would have occurred.
    pub fn execution_price(&self) -> Price { self.execution_price }

    /// AIQ strategy used on the cancelled order.
    pub fn aiq_strategy(&self) -> AiqStrategy { self.aiq_strategy }

    /// The number of shares just decremented from the order. 
    /// This number is incremental, not cumulative.
    pub fn decrement_shares(&self) -> u32 { self.decrement_shares }

    /// Shares that would have executed if the trade had occurred.
    pub fn quantity_prevented(&self) -> u32 { self.quantity_prevented }
    
    /// For an AIQ Cancel message, this value will always be "Q".
    pub fn reason(&self) -> CancelReason { self.reason }
    
    /// Get read-only access to the message's optional fields.
    pub fn options(&self) -> &Vec<TagValue> {
        &self.optional_appendage.tag_values()
    }

}


///
#[derive(Debug, Clone)]
pub struct CancelPending {
    timestamp: NaiveTime,
    user_ref_num: UserRefNum,
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
    
    pub fn timestamp(&self) -> NaiveTime { self.timestamp }

    /// Gets the user reference number.
    pub fn user_ref_num(&self) -> UserRefNum { self.user_ref_num }

    /// Get read-only access to the message's optional fields.
    pub fn options(&self) -> &Vec<TagValue> {
        &self.optional_appendage.tag_values()
    }

}


///
#[derive(Debug, Clone)]
pub struct CancelRejected {
    timestamp: NaiveTime,
    user_ref_num: UserRefNum,
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
    
    pub fn timestamp(&self) -> NaiveTime { self.timestamp }

    /// Gets the user reference number.
    pub fn user_ref_num(&self) -> UserRefNum { self.user_ref_num }

    /// Get read-only access to the message's optional fields.
    pub fn options(&self) -> &Vec<TagValue> {
        &self.optional_appendage.tag_values()
    }

}


///
#[derive(Debug, Clone)]
pub struct MassCancelResponse {
    timestamp: NaiveTime,
    user_ref_num: UserRefNum,
    firm: FirmId,
    symbol: StockSymbol,
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
    
    pub fn timestamp(&self) -> NaiveTime { self.timestamp }

    /// Gets the user reference number.
    pub fn user_ref_num(&self) -> UserRefNum { self.user_ref_num }

    /// Gets the ID for the firm for whom the orders will be canceled.
    pub fn firm(&self) -> &FirmId { &self.firm }
    
    /// Gets the symbol for which the orders will be canceled.
    pub fn symbol(&self) -> &StockSymbol { &self.symbol }
    
    /// Get read-only access to the message's optional fields.
    pub fn options(&self) -> &Vec<TagValue> {
        &self.optional_appendage.tag_values()
    }

}


