
use nsdq_util::define_enum;

define_enum! {

    LiquidityFlag: 
        "Specifies whether the order contributed liquidity as a maker \
        (i.e., added to the order book) or removed liquidity as a taker \
        (i.e., matched against an existing order in the book). \n \
        Adding liquidity often earns a rebate, removing it often incurs fees.";

    ['A'] Added 
        "Order added liquidity to the order book. (All markets)",
    ['R'] Removed 
        "Order removed liquidity from the order book. (All markets)",
    ['0'] Supplemental 
        "Order added liquidity as part of a supplemental liquidity event, \
        typically outside standard continuous trading. (NASDAQ only)",
    ['O'] OpeningCross 
        "Order executed during the Opening Cross. \
        Typically incurs specific auction fees. (NASDAQ only)",
    ['C'] ClosingCross 
        "Order executed during the Closing Cross. \
        Typically incurs specific auction fees. (NASDAQ only)",
    ['M'] ImbalanceOpeningCross 
        "Order participated in the Opening Cross specifically to address an \
        order imbalance (e.g., excess buy or sell orders). \n \
        May receive favorable fees/rebates for correcting imbalances. \
        (NASDAQ only)",
    ['L'] ImbalanceClosingCross 
        "Order participated in the Closing Cross specifically to address an \
        order imbalance (e.g., excess buy or sell orders). \n \
        May receive favorable fees/rebates for correcting imbalances. \
        (NASDAQ only)",
    ['i'] AfterHoursClosingCross 
        "Order executed during the After-hours Closing Cross. \
        Fees may differ due to lower volume and unique timing. \
        (NASDAQ only)",
    ['K'] HaltCross 
        "Order executed during a Halt Cross, \
        used to reopen trading after a regulatory or volatility halt. \n \
        Fees may be unique due to the special nature of halts. \
        (NASDAQ only)",
    ['H'] HaltIpoCross 
        "Order executed during an IPO Cross or Halt Cross, \
        (used for reopenings after significant halts). \n \
        Often has distinct fee structures due to high visibility and volume. \
        (NASDAQ only)",
    ['k'] MidpointOrderAdded 
        "Midpoint order added liquidity by resting at the midpoint of the \
        National Best Bid and Offer (NBBO) across all markets.",
    ['m'] MidpointOrderRemoved 
        "Order that removed liquidity at the midpoint of the NBBO. \
        (NASDAQ/BX only)",
    ['N'] PassiveMidpoint 
        "Marks a passive midpoint execution, where a non-aggressive order \
        (e.g., a resting midpoint order) executes. (NASDAQ/BX only)",
    ['n'] MidpointExtendedLife 
        "Order executed was a Midpoint Extended Life Order (M-ELO). \
        (NASDAQ only)",
    ['J'] NonDisplayedAdded 
        "Order added liquidity without being visible. (All markets)",
    ['u'] ReserveNonDisplayedAdded 
        "Reserve order ('iceberg') added non-displayed liquidity, \
        where only a portion is shown, and the rest is hidden. \
        (All markets)",
    ['e'] RetailAdded 
        "Retail-designated execution that added displayed liquidity. \
        (NASDAQ/BX only)",
    ['j'] RetailPriceImproving 
        "Retail Price Improving (RPI) order provided liquidity at a price 
        better than the NBBO, often earning a rebate. (BX only)",
    ['p'] RemovedPriceImprovingNonDisplayed 
        "Order removed non-displayed liquidity improving the NBBO. (BX only)",
    ['q'] RmoRemovesNonRpiMidpoint 
        "Retail Market Order (RMO) removed non-RPI midpoint liquidity. \
        (BX only)",
    ['r'] RetailRemovesRpi 
        "Retail Order removes RPI liquidity (BX only)",
// TODO: `alt` in parser has max of 21 chars, switch from tuple to array
/*
    ['t'] RetailRemovesNonDisplayed 
        "Retail Order removes price improving non-displayed liquidity \
        other than RPI liquidity. (BX only)",
    ['7'] NbboImprovedAdded 
        "Displayed, liquidity-adding order improves the NBBO. \
        (NASDAQ/BX only)",
    ['8'] QbboSetWhileJoining 
        "Displayed, liquidity-adding order sets the QBBO while joining. \
        (NASDAQ/BX only the NBBO)",
    ['1'] RpiNonRpii 
        "RPI order provides liquidity, No RPII. (BX only)",
*/
}

