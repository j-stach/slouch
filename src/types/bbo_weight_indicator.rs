
use crate::error::BadElementError;

///
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BboWeightIndicator {

    /// 0% - 0.2%
    Micro,

    /// 0.2% - 1%
    Small,

    /// 1% - 2%
    Medium,

    /// >2%
    Large,

    /// Sets the QBBO while joining the NBBO.
    SetQbboWhileJoin,

    /// Improves the NBBO upon entry.
    ImproveNbboOnEntry,

    /// Unspecified (default)
    Unspecified,

}

impl BboWeightIndicator {

    pub(crate) fn parse(data: u8) -> Result<Self, BadElementError> {

        use Self::*;
        match data {
            b'0' => Ok(Micro),
            b'1' => Ok(Small),
            b'2' => Ok(Medium),
            b'3' => Ok(Large),
            b'S' => Ok(SetQbboWhileJoin),
            b'N' => Ok(ImproveNbboOnEntry),
            b' ' => Ok(Unspecified),

            _ => BadElementError::InvalidEnum(
                (data as char).to_string(), 
                "BboWeightIndicator".to_string()
            )
        }
    }

    pub(crate) fn encode(&self) -> u8 {
        
        use Self::*;
        match self {
            Micro               => b'0',
            Small               => b'1',
            Medium              => b'2',
            Large               => b'3',
            SetQbboWhileJoin    => b'S',
            ImproveNbboOnEntry  => b'N',
            Unspecified         => b' ',
        }
    }

}
