
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


/// Informs you that an order has been reduced or canceled.
/// OrderCanceled does not necessarily mean the entire order is dead; 
/// some portion of the order may still be alive.
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

        if data.len() < 17 {
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
            optional_appendage: if data.len() >= 17 {
                OptionalAppendage::parse(&data[17..])?
            } else {
                OptionalAppendage::new()
            }
        })
    }
    
    /// Time this message was generated.
    pub fn timestamp(&self) -> &NaiveTime { &self.timestamp }

    /// Refers to the order that was cancelled.
    pub fn user_ref_num(&self) -> UserRefNum { self.user_ref_num }

    /// The number of shares being decremented from the order. 
    /// This number is incremental, not cumulative.
    pub fn quantity(&self) -> u32 { self.quantity }
    
    /// Reason why the order was canceled.
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

        if data.len() < 31 {
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
            optional_appendage: if data.len() >= 31 {
                OptionalAppendage::parse(&data[31..])?
            } else {
                OptionalAppendage::new()
            }
        })
    }
    
    /// Time this message was generated.
    pub fn timestamp(&self) -> NaiveTime { self.timestamp }

    /// Refers to the order that was cancelled.
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


/// Sent in response to a CancelOrder request for a cross order during a 
/// pre-cross late period signifying that it cannot be canceled at this time, 
/// but any unexecuted portion of this order will automatically be canceled 
/// immediately after the cross completes.
///
/// This message will only be sent once for a given UserRefNum. 
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

        if data.len() < 12 {
            return Err(OuchError::Parse("CancelPending".to_string()))
        }

        Ok(Self {
            timestamp: {
                let ts = u64_from_be_bytes(&data[0..=7])?;
                nanosec_from_midnight(ts)
            },
            user_ref_num: UserRefNum::parse(&data[8..=11])?,
            optional_appendage: if data.len() >= 12 {
                OptionalAppendage::parse(&data[12..])?
            } else {
                OptionalAppendage::new()
            }
        })
    }
    
    /// Time this message was generated.
    pub fn timestamp(&self) -> NaiveTime { self.timestamp }

    /// Refers to the order that is pending cancellation.
    pub fn user_ref_num(&self) -> UserRefNum { self.user_ref_num }

    /// Get read-only access to the message's optional fields.
    pub fn options(&self) -> &Vec<TagValue> {
        &self.optional_appendage.tag_values()
    }

}


/// Sent in response to a partial cancel request (with non-zero “quantity”) 
/// for a cross order during a pre-cross late period signifying that it 
/// cannot be partially canceled at this time. 
///
/// No automatic cancel will be scheduled for this order. 
/// Clients could repeat their request for any unexecuted portion of the order 
/// after the cross completes.
///
/// This message will only be sent once for a given UserRefNum. 
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

        if data.len() < 12 {
            return Err(OuchError::Parse("CancelRejected".to_string()))
        }

        Ok(Self {
            timestamp: {
                let ts = u64_from_be_bytes(&data[0..=7])?;
                nanosec_from_midnight(ts)
            },
            user_ref_num: UserRefNum::parse(&data[8..=11])?,
            optional_appendage: if data.len() >= 12 {
                OptionalAppendage::parse(&data[12..])?
            } else {
                OptionalAppendage::new()
            }
        })
    }
    
    /// Time this message was generated.
    pub fn timestamp(&self) -> NaiveTime { self.timestamp }

    /// Refers to the order that could not be cancelled.
    pub fn user_ref_num(&self) -> UserRefNum { self.user_ref_num }

    /// Get read-only access to the message's optional fields.
    pub fn options(&self) -> &Vec<TagValue> {
        &self.optional_appendage.tag_values()
    }

}


/// Acknowledges the receipt of a valid MassCancel request. 
/// The data fields from the request are echoed back in this message.
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
    
    /// Time this message was generated.
    pub fn timestamp(&self) -> NaiveTime { self.timestamp }

    /// Refers to the MassCancel request.
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


