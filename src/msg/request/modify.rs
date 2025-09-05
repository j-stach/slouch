
use crate::types::{
    UserRefNum,
    Side
};
use crate::msg::options::OptionalAppendage;

///
#[derive(Debug, Serialize, Deserialize)]
pub struct ModifyOrder {
    pub user_ref_num: UserRefNum,
    pub side: Side,
    pub quantity: u32,
    optional_appendage: OptionalAppendage
}

impl ModifyOrder {
    
    pub(super) fn encode(&self) -> Vec<u8> {

        let mut bytes: Vec<u8> = Vec::new();

        bytes.push(b'X');
        bytes.extend(self.side.encode());
        bytes.extend(self.quantity.to_be_bytes());
        bytes.extend(self.optional_appendage.encode());

        bytes
    }
} 

