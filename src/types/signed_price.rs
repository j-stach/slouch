
use crate::{
    helper::i32_from_be_bytes,
    error::BadElementError,
};

///
// TODO: Send email to ask if there are any limits to Peg Offsets
// that I should hardcode into the SignedPrice type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SignedPrice {

    negative: bool,
    //
    dollars: u32,
    // 
    cents: u16
}

/// Public functions that can be used to create price values.
impl SignedPrice {

    ///
    // TODO: Send email to ask if there are any limits to Peg Offsets
    // that I should hardcode into the SignedPrice type.
    pub fn new(
        dollars: u32, 
        cents: u16,
        negative: bool
    ) -> Result<Self, BadElementError> {

        // TODO: Ensure SignedPrice is within limits?
        if cents > 9999 { 
            return Err(BadElementError::InvalidValue("SignedPrice".to_string()))
        }

        Ok(SignedPrice { negative, dollars, cents })
    }

}

impl SignedPrice {

    pub(crate) fn encode(&self) -> [u8; 4] {
        // OUCH price has four decimals implied.
        let mut price: i32 = self.dollars as i32 * 10_000 + self.cents as i32;
        if self.negative { price = price * -1 }
        price.to_be_bytes()
    }

    pub(crate) fn parse(data: &[u8]) -> Result<Self, BadElementError> {

        let mut price = i32_from_be_bytes(&data)?;

        // Extract the sign.
        let negative = price < 0;
        price = price.abs();

        // OUCH price has four decimals implied.
        SignedPrice::new(
            (price / 10_000) as u32, 
            (price % 10_000) as u16,
            negative
        )
    }
}

