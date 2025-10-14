
use crate::error::BadElementError;

/// Specifies whether the order contributed liquidity as a maker
/// (i.e., added to the order book) 
/// or removed liquidity as a taker
/// (i.e., matched against an existing order in the book).
/// Adding liquidity often earns a rebate, and removing it often incurs a fee.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LiquidityFlag {

    /// Order added liquidity to the order book. (All markets)
    Added,

    /// Order removed liquidity from the order book. (All markets)
    Removed,

    /// Order added liquidity as part of a supplemental liquidity event, 
    /// typically outside standard continuous trading. (NASDAQ only)
    Supplemental,

    /// Order executed during the Opening Cross.
    /// Typically incurs specific auction fees. (NASDAQ only)
    OpeningCross,

    /// Order executed during the Closing Cross.
    /// Typically incurs specific auction fees. (NASDAQ only)
    ClosingCross,

    /// Order participated in the Opening Cross specifically to address an 
    /// order imbalance (e.g., excess buy or sell orders).
    /// May receive favorable fees/rebates for correcting imbalances.
    /// (NASDAQ only)
    ImbalanceOpeningCross,

    /// Order participated in the Closing Cross specifically to address an 
    /// order imbalance (e.g., excess buy or sell orders).
    /// May receive favorable fees/rebates for correcting imbalances.
    /// (NASDAQ only)
    ImbalanceClosingCross,

    /// Order executed during the After-hours Closing Cross.
    /// Fees may differ due to lower volume and unique timing.
    /// (NASDAQ only)
    AfterHoursClosingCross,

    /// Order executed during a Halt Cross, 
    /// used to reopen trading after a regulatory or volatility halt.
    /// Fees may be unique due to the special nature of halts.
    /// (NASDAQ only)
    HaltCross,

    /// Order executed during an IPO Cross or Halt Cross, 
    /// (used for reopenings after significant halts).
    /// Often has distinct fee structures due to high visibility and volume.
    /// (NASDAQ only)
    HaltIpoCross,

    /// Midpoint order added liquidity by resting at the midpoint of the 
    /// National Best Bid and Offer (NBBO) across all markets.
    MidpointOrderAdded,

    /// Order that removed liquidity at the midpoint of the NBBO.
    /// (NASDAQ/BX only)
    MidpointOrderRemoved,

    /// Marks a passive midpoint execution, where a non-aggressive order 
    /// (e.g., a resting midpoint order) executes. (NASDAQ/BX only)
    PassiveMidpoint,

    /// Order executed was a Midpoint Extended Life Order (M-ELO).
    /// (NASDAQ only)
    MidpointExtendedLife,

    /// Order added liquidity without being visible. (All markets)
    NonDisplayedAdded,

    /// Reserve order ("iceberg") added non-displayed liquidity, 
    /// where only a portion is shown, and the rest is hidden.
    /// (All markets)
    ReserveNonDisplayedAdded,

    /// Retail-designated execution that added displayed liquidity.
    /// (NASDAQ/BX only)
    RetailAdded,

    /// Retail Price Improving (RPI) order provided liquidity at a price 
    /// better than the NBBO, often earning a rebate. (BX only)
    RetailPriceImproving,

    /// Order removed non-displayed liquidity at a price better than the NBBO.
    /// (BX only)
    RemovedPriceImprovingNonDisplayed,

    /// Retail Market Order (RMO) removed non-RPI midpoint liquidity. 
    /// (BX only)
    RmoRemovesNonRpiMidpoint,

    /// Retail Order removes RPI liquidity (BX only)
    RetailOrderRemovesRpi,

    /// Retail Order removes price improving non-displayed liquidity
    /// other than RPI liquidity. (BX only)
    RetailOrderRemovesNonDisplayedNonRpi,

    /// Displayed, liquidity-adding order improves the NBBO. 
    /// (NASDAQ/BX only)
    NbboImprovedAdded,

    /// Displayed, liquidity-adding order sets the QBBO while joining.
    /// (NASDAQ/BX only the NBBO)
    QbboSetWhileJoining,

    /// RPI order provides liquidity, No RPII. (BX only)
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

    #[allow(dead_code)] // Future use
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
