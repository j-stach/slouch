
use crate::types::{
    UserRefNum,
    Firm
};
use crate::msg::options::OptionalAppendage;


///
#[derive(Debug, Serialize, Deserialize)]
pub struct DisableOrderEntry {
    pub user_ref_num: UserRefNum,
    pub firm: Firm,
    optional_appendage: OptionalAppendage
}

impl DisableOrderEntry {
    
    pub(super) fn encode(&self) -> Vec<u8> {

        let mut bytes: Vec<u8> = Vec::new();

        bytes.push(b'D');
        bytes.extend(self.user_ref_num.encode());
        bytes.extend(self.firm.encode());
        bytes.extend(self.optional_appendage.encode());

        bytes
    }
} 

///
#[derive(Debug, Serialize, Deserialize)]
pub struct EnableOrderEntry {
    pub user_ref_num: UserRefNum,
    pub firm: Firm,
    optional_appendage: OptionalAppendage
}

impl EnableOrderEntry {
    
    pub(super) fn encode(&self) -> Vec<u8> {

        let mut bytes: Vec<u8> = Vec::new();

        bytes.push(b'E');
        bytes.extend(self.user_ref_num.encode());
        bytes.extend(self.firm.encode());
        bytes.extend(self.optional_appendage.encode());

        bytes
    }
} 

