
use crate::error::BadElementError;

///
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LiquidityFlag {

    /// Added (All markets)
    Added,

    /// Removed (All markets)
    Removed,

    /// Supplemental order execution (NASDAQ only)
    Supplemental,

    /// Opening cross (NASDAQ only)
    OpeningCross,

    /// Closing cross (NASDAQ only)
    ClosingCross,

    /// Opening Cross (imbalance-only) (NASDAQ only)
    ImbalanceOpeningCross,

    /// Closing Cross (imbalance-only) (NASDAQ only)
    ImbalanceClosingCross,

    /// After hours closing cross (NASDAQ only)
    AfterHoursClosingCross,

    /// Halt cross (NASDAQ only)
    HaltCross,

    /// Halt/IPO cross (NASDAQ only)
    HaltIpoCross,

    /// Added liquidity via a midpoint order (All markets)
    MidpointOrderAdded,

    /// Removed liquidity at a midpoint (NASDAQ/BX only)
    MidpointOrderRemoved,

    /// Passive midpoint execution (NASDAQ/BX only)
    PassiveMidpoint,

    /// Midpoint Extended Life Order execution (NASDAQ only)
    MidpointExtendedLife,

    /// Non-displayed adding liquidity (All markets)
    NonDisplayedAdded,

    /// Added non-displayed liquidity via a Reserve order (All markets)
    ReserveNonDisplayedAdded,

    /// Retail designated execution that added displayed liquidity
    /// (NASDAQ/BX only)
    RetailAdded,

    /// RPI (Retail Price Improving) order provides liquidity (BX only)
    RetailPriceImproving,

    /// Removed price improving non-displayed liquidity (BX only)
    RemovedPriceImprovingNonDisplayed,

    /// RMO Retail Order removes non-RPI midpoint liquidity (BX only)
    RmoRemovesNonRpiMidpoint,

    /// Retail Order removes RPI liquidity (BX only)
    RetailOrderRemovesRpi,

    /// Retail Order removes price improving non-displayed liquidity
    /// other than RPI liquidity (BX only)
    RetailOrderRemovesNonDisplayedNonRpi,

    /// Displayed, liquidity-adding order improves the NBBO 
    /// (NASDAQ/BX only)
    NbboImprovedAdded,

    /// Displayed, liquidity-adding order sets the QBBO while joining
    /// (NASDAQ/BX only the NBBO)
    QbboSetWhileJoining,

    /// RPI order provides liquidity, No RPII (BX only)
    RpiNonRpii,

}

impl LiquidityFlag {

    pub(crate) fn parse(data: u8) -> Result<Self, BadElementError> {

        use LiquidityFlag::*;
        match data {
            b'A' => Ok(Added),
            b'R' => Ok(Removed),
            b'0' => Ok(Supplemental),
            b'O' => Ok(OpeningCross),
            b'C' => Ok(ClosingCross),
            b'M' => Ok(ImbalanceOpeningCross),
            b'L' => Ok(ImbalanceClosingCross),
            b'i' => Ok(AfterHoursClosingCross),
            b'K' => Ok(HaltCross),
            b'H' => Ok(HaltIpoCross),
            b'k' => Ok(MidpointOrderAdded),
            b'm' => Ok(MidpointOrderRemoved),
            b'N' => Ok(PassiveMidpoint),
            b'n' => Ok(MidpointExtendedLife),
            b'J' => Ok(NonDisplayedAdded),
            b'u' => Ok(ReserveNonDisplayedAdded),
            b'e' => Ok(RetailAdded),
            b'j' => Ok(RetailPriceImproving),
            b'p' => Ok(RemovedPriceImprovingNonDisplayed),
            b'q' => Ok(RmoRemovesNonRpiMidpoint),
            b'r' => Ok(RetailOrderRemovesRpi),
            b't' => Ok(RetailOrderRemovesNonDisplayedNonRpi),
            b'7' => Ok(NbboImprovedAdded),
            b'8' => Ok(QbboSetWhileJoining),
            b'1' => Ok(RpiNonRpii),

            _ => Err(BadElementError::InvalidEnum(
                (data as char).to_string(), 
                "LiquidityFlag".to_string()
            ))
        }
    }

    pub(crate) fn encode(&self) -> u8 {
        
        use LiquidityFlag::*;
        match self {

             Added                                => b'A',
             Removed                              => b'R',
             Supplemental                         => b'0',
             OpeningCross                         => b'O',
             ClosingCross                         => b'C',
             ImbalanceOpeningCross                => b'M',
             ImbalanceClosingCross                => b'L',
             AfterHoursClosingCross               => b'i',
             HaltCross                            => b'K',
             HaltIpoCross                         => b'H',
             MidpointOrderAdded                   => b'k',
             MidpointOrderRemoved                 => b'm',
             PassiveMidpoint                      => b'N',
             MidpointExtendedLife                 => b'n',
             NonDisplayedAdded                    => b'J',
             ReserveNonDisplayedAdded             => b'u',
             RetailAdded                          => b'e',
             RetailPriceImproving                 => b'j',
             RemovedPriceImprovingNonDisplayed    => b'p',
             RmoRemovesNonRpiMidpoint             => b'q',
             RetailOrderRemovesRpi                => b'r',
             RetailOrderRemovesNonDisplayedNonRpi => b't',
             NbboImprovedAdded                    => b'7',
             QbboSetWhileJoining                  => b'8',
             RpiNonRpii                           => b'1',
        }
    }

}
