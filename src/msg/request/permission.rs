
#[derive(Debug, Serialize, Deserialize)]
pub struct DisableOrderEntry {
    /// Must be day-unique and strictly increasing for each OUCH account.
    // TODO: Client should track and increment this
    // TODO: Data type
    user_ref_num: u32,
    // TODO: All caps 4 char string
    firm: Firm,
    /// This holds all optional fields included in the order.
    optional_appendage: OptionalAppendage
}

impl DisableOrderEntry {
    
    pub(super) fn encode(&self) -> Vec<u8> {
        todo!{}
    }
} 

#[derive(Debug, Serialize, Deserialize)]
pub struct EnableOrderEntry {
    /// Must be day-unique and strictly increasing for each OUCH account.
    // TODO: Client should track and increment this
    // TODO: Data type
    user_ref_num: u32,
    // TODO: All caps 4 char string
    firm: Firm,
    /// This holds all optional fields included in the order.
    optional_appendage: OptionalAppendage
}

impl EnableOrderEntry {
    
    pub(super) fn encode(&self) -> Vec<u8> {
        todo!{}
    }
} 

