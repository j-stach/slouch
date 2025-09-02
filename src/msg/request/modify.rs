
use crate::types::UserRefNum;
use super::options::OptionalAppendage;

///
#[derive(Debug, Serialize, Deserialize)]
pub struct ModifyOrder {
    /// Must be day-unique and strictly increasing for each OUCH account.
    // TODO: Client should track and increment this
    // TODO: Data type
    user_ref_num: u32,
    // TODO: Use Enum for Buy, Sell, Sell short, Sell short exempt
    side: char,
    /// Number of shares.
    quantity: u32,
    /// This holds all optional fields included in the order.
    optional_appendage: OptionalAppendage
}

impl ModifyOrder {
    
    pub(super) fn encode(&self) -> Vec<u8> {
        todo!{}
    }
} 

