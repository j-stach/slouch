
use crate::error::BadElementError;


/// Trading capacity with which an order will be placed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Capacity {

    /// The firm is acting as an agent on behalf of a customer.
    Agency,

    /// The firm is trading for its own account, assuming market risk.
    Principal,

    /// The firm acts as an intermediary in a matched trade without risk 
    /// (e.g., simultaneously buying and reselling at the same price).
    Riskless,

    /// Used for capacities that don't fit the above categories.
    Other
}

impl Capacity {

    pub(crate) fn encode(&self) -> u8 {

        use Capacity::*;
        match self {
            Agency => b'A',
            Principal => b'P',
            Riskless => b'R',
            Other => b'O',
        }
    }

    pub(crate) fn parse(data: u8) -> Result<Self, BadElementError> {

        use Capacity::*;
        match data {
            b'A' => Ok(Agency),
            b'P' => Ok(Principal),
            b'R' => Ok(Riskless),
            b'O' => Ok(Other),

            _ => Err(BadElementError::InvalidEnum(
                (data as char).to_string(), 
                "Capacity".to_string()
            ))
        }
    }
}

