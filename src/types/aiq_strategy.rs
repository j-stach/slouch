
use crate::error::BadElementError;

/// I am trusting the user to know what these mean.
/// If you do not know, then chances are you will not need them.
/// https://www.nasdaqtrader.com/TraderNews.aspx?id=ETU2023-8
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AiqStrategy {
    CancelOldestMpid,
    CancelNewestMpid,
    DecrementBothMpid,
    DecrementBothNoDetailsMpid,
    CancelOldestOrgId,
    CancelNewestOrgId,
    DecrementBothOrgId,
    DecrementBothNoDetailsOrgId,
}

impl AiqStrategy {

    pub(crate) fn parse(data: u8) -> Result<Self, BadElementError> {

        use AiqStrategy::*;
        match data {

            b'O' => Ok(CancelOldestMpid),
            b'W' => Ok(CancelNewestMpid),
            b'D' => Ok(DecrementBothMpid),
            b'Y' => Ok(DecrementBothNoDetailsMpid),
            b'o' => Ok(CancelOldestOrgId),
            b'w' => Ok(CancelNewestOrgId),
            b'd' => Ok(DecrementBothOrgId),
            b'y' => Ok(DecrementBothNoDetailsOrgId),

            _ => Err(BadElementError::InvalidEnum(
                (data as char).to_string(), 
                "AiqStrategy".to_string()
            ))
        }
    }

    #[allow(dead_code)] // Future use
    pub(crate) fn encode(&self) -> u8 {
        
        use AiqStrategy::*;
        match self {

             CancelOldestMpid               => b'O',
             CancelNewestMpid               => b'W',
             DecrementBothMpid              => b'D',
             DecrementBothNoDetailsMpid     => b'Y',
             CancelOldestOrgId              => b'o',
             CancelNewestOrgId              => b'w',
             DecrementBothOrgId             => b'd',
             DecrementBothNoDetailsOrgId    => b'y',
        }
    }

}
