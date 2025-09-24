
use crate::error::BadElementError;

/// I am trusting the user to know what these mean.
/// If you do not know, then chances are you will not need them.
/// https://www.nasdaqtrader.com/TraderNews.aspx?id=ETU2023-8
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

        use Self::*;
        match data {

            b'O' => Ok(CancelOldestMpid),
            b'W' => Ok(CancelNewestMpid),
            b'D' => Ok(DecrementBothMpid),
            b'Y' => Ok(DecrementBothNoDetailsMpid),
            b'o' => Ok(CancelOldestOrgId),
            b'w' => Ok(CancelNewestOrgId),
            b'd' => Ok(DecrementBothOrgId),
            b'y' => Ok(DecrementBothNoDetailsOrgId),

            _ => BadElementError::InvalidEnum(
                data as char, 
                "AiqStrategy".to_string()
            )
        }
    }

    pub(crate) fn encode(&self) -> u8 {
        
        use Self::*;
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
