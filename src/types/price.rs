
use crate::helper::u32_from_be_bytes;


///
pub struct Price {
    /// Maximum accepted value is $199,999.9900
    dollars: u32,
    /// Integer to represent the decimal portion of price
    /// 9,900 = $0.99
    cents: u16
}

/// Public functions that can be used to create price values.
impl Price {

    /// Checks if custom price is a valid value (i.e., < $199,999.9900).
    /// Helps ensure message encoding is done correctly.
    pub fn new(dollars: u32, cents: u16) -> Result<Self, BadElementError> {

        // Ensures price is within limits.
        if dollars > 199_999 || (dollars == 199_999 && cents >= 9900) {
            // TODO:
            // return BadElementError::InvalidValue
        }

        Ok(Price { dollars, cents })
    }

    /// Use the special price of $200,000.0000 to flag an order as a market order.
    pub fn market() -> Self {
        Price { 
            dollars: 200_000,
            cents: 0000,
        }
    }

    /// Use the special price of $214,748.3647 to flag an order as a market order for a cross.
    pub fn market_cross() -> Self {
        Price {
            dollars: 214_748,
            cents: 3647,
        }
    }
}

impl Price {

    pub(crate) fn encode(&self) -> Vec<u8> {
        // OUCH price has four decimals implied.
        let price = self.dollars * 10_000 + self.cents;
        price.to_be_bytes()
    }

    pub(crate) fn parse(data: Vec<u8>) -> Result<Self, BadElementError> {

        let price = u32_from_be_bytes(&data)?;
        // OUCH price has four decimals implied.
        Price::new(price / 10_000, price % 10_000)
    }
}

