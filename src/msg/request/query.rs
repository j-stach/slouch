
///
#[derive(Debug, Serialize, Deserialize)]
pub struct AccountQuery {
    /// This holds all optional fields included in the order.
    optional_appendage: OptionalAppendage
}

impl AccountQuery {
    
    pub(super) fn encode(&self) -> Vec<u8> {
        todo!{}
    }
} 

