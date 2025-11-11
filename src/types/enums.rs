
use nsdq_util::define_enum;

define_enum!{

    AiqStrategy: 
        "I am trusting the user to know what these mean. \
        If you do not know, then chances are you will not need them. \
        https://www.nasdaqtrader.com/TraderNews.aspx?id=ETU2023-8";

    ['O'] CancelOldestMpid,
    ['W'] CancelNewestMpid,
    ['D'] DecrementBothMpid,
    ['Y'] DecrementBothNoDetailsMpid,
    ['o'] CancelOldestOrgId,
    ['w'] CancelNewestOrgId,
    ['d'] DecrementBothOrgId,
    ['y'] DecrementBothNoDetailsOrgId,
}

define_enum!{

    BboWeight: 
        "Indicates the weighting of the order in the Best Bid and Offer (BBO).";

    ['0'] Micro "0% - 0.2%",
    ['1'] Small "0.2% - 1%",
    ['2'] Medium "1% - 2%",
    ['3'] Large ">2%",
    ['S'] SetQbboWhileJoin "Sets the QBBO while joining the NBBO.",
    ['N'] ImproveNbboOnEntry "Improves the NBBO upon entry.",
    [' '] Unspecified "Unspecified (default)",
}

define_enum!{

    BrokenReason: "Reason a trade execution was broken.";

    ['E'] Erroneous "Trade deemed 'clearly erroneous.'",
    ['C'] Consent "Both parties agreed to break the trade.",
    ['S'] Supervisory "Manually broken by NASDAQ supervisory terminal.",
    ['X'] External "Broken by external third party.",
}

define_enum! {

    CancelReason: "Reason an order was cancelled (`CancelOrder` message).";

    ['D'] RegulatoryRestriction 
        "The order cannot be executed because of a regulatory restriction \
        (e.g., trade through restrictions).",
    ['E'] Closed 
        "Any `TimeInForce::Day` order that was received after the closing \
        cross is complete in a given symbol will receive this reason.",
    ['F'] PostOnlyNms 
        "Post Only order would have been price slid for NMS.",
    ['G'] PostOnlySlide 
        "Post Only order would have been price slid due to a \
        contra side displayed order on the book.",
    ['H'] Halted 
        "On-open order was canceled because the symbol remained halted \
        after the opening cross completed.",
    ['I'] Immediate 
        "Immediate or Cancel Order.",
    ['K'] MarketCollars 
        "This order cannot be executed because of Market Collars.",
    ['Q'] SelfMatchPrevention 
        "The order was cancelled because it would have executed with an \
        existing order entered by the same MPID.",
    ['S'] Supervisory 
        "The order was manually canceled by a NASDAQ supervisory terminal.",
    ['T'] Timeout 
        "The Time In Force for the order has expired.",
    ['U'] UserRequested 
        "Sent in response to a CancelOrder request.",
    ['X'] OpenProtection 
        "The order was cancelled by the Opening Price Protection Threshold.",
    ['Z'] System 
        "This order was cancelled by the system.",
    ['e'] ExceedsAllowableShares 
        "Company Direct Listing Capital Raise order exceeds allowable shares.",
}

define_enum!{

    Capacity: "Trading capacity with which an order will be placed.";

    ['A'] Agency 
        "The firm is acting as an agent on behalf of a customer.",
    ['P'] Principal 
        "The firm is trading for its own account, assuming market risk.",
    ['R'] Riskless 
        "The firm acts as an intermediary in a matched trade without risk \
        (e.g., simultaneously buying and reselling at the same price).",
    ['O'] Other 
        "Used for capacities that don't fit the above categories.",
}

define_enum!{

    CrossType: 
        "Designates the specific market session, auction, \
        or execution mechanism the order should participate in, \
        influencing when and how it executes.";

    ['N'] ContinuousMarket 
        "Standard limit order for ongoing matching in the regular trading book.\
        (NASDAQ Only)",
    ['O'] Opening 
        "Participates in auction to determine the official opening price. \
        (NASDAQ Only)",
    ['C'] Closing 
        "Joins the closing auction for the official close. (NASDAQ Only)",
    ['H'] Halt 
        "Reopenings after halts or initial public offerings. (NASDAQ Only)",
    ['S'] Supplemental 
        "Provides additional liquidity during crosses. (NASDAQ Only)",
    ['R'] Retail 
        "Specific to BX exchange for retail-designated orders. (BX Only)",
    ['E'] ExtendedLife 
        "For orders with longer persistence. (NASDAQ Only)",
    ['A'] AfterHoursClose 
        "For after-hours closing mechanisms. (NASDAQ Only)",
}

define_enum! {

    Display: "Visibility options for an order.";

    ['Y'] Visible 
        "The order is fully displayed in the public order book, \
        showing the full quantity at the specified price.",
    ['N'] Hidden 
        "The order is not visible in the public book but can still match \
        against incoming orders.",
    ['A'] Attributable 
        "The order is visible but attributed to a specific \
        Market Participant Identifier (MPID), rather than being anonymous.",
}

define_enum!{
    
    EventCode: "Identifier for a system-wide signal.";

    ['S'] StartOfDay 
        "Indicates that exchange is open and is ready to accept orders.",
    ['E'] EndOfDay 
        "Exchange is closed and will no longer accept new orders. \n \
        NOTE: Still possible to receive breaks and cancels after this event.",
}

define_enum!{

    HandleInst: 
        "Handling instructions for routing and execution behavior \
        (e.g., liquidity sourcing and interaction with other markets).";

    ['I'] ImbalanceOnly 
        "Imbalance only (CrossType::Opening & CrossType::Closing)",
    ['O'] RetailOrder1 
        "Retail order type 1 (CrossType::Retail)",
    ['T'] RetailOrder2 
        "Retail order type 2 (CrossType::Retail)",
    ['Q'] RetailPriceImprovement 
        "Retail price improvement (CrossType::Retail)",
    ['B'] ExtendedContinuous 
        "Extended Life + Continuous (CrossType::ExtendedLife)",
    ['D'] DirectListingCapitalRaise 
        "Direct Listing Capital Raise (CrossType::Halt)",
    [' '] NoInstructions 
        "No handling instructions.",
}

impl Default for HandleInst {
    fn default() -> Self {
        HandleInst::NoInstructions
    }
}

define_enum! {

    Liquidity: 
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
}

define_enum!{

    OrderState: 
        "Should be a boolean, but in the interest of maintaining consistency \
        with the wording in the spec, we use an enum.";

    ['L'] Live "Order is accepted and tracked in the system.",
    ['D'] Dead "Means that the order was accepted and automatically canceled.",
}

define_enum!{

    PriceType: "Defines how the order's Price will be interpreted.";

    ['L'] Limit 
        "Standard limit order with a fixed price set by the trader. \
        The order executes only at or better than the specified price.",
    ['P'] MarketPeg 
        "Specifies a pegged order that tracks the best bid (for sell orders) \
        or best offer (for buy orders) of the National Best Bid and Offer \
        (NBBO), typically with an optional offset.",
    ['M'] MidpointPeg 
        "Specifies a pegged order that tracks the midpoint of the NBBO. \n \
        The order is typically non-displayed and executes at the midpoint, \
        offering price improvement.",
    ['R'] PrimaryPeg 
        "Specifies a pegged order that tracks the primary market’s quote, \
        often with an offset. \n \
        Unlike MarketPeg, it focuses on the primary exchange’s quote \
        rather than the consolidated NBBO.",
    ['Q'] MarketMakerPeg 
        "Specifies a pegged order designed for market makers, \
        typically tracking a reference price (e.g., NBBO or primary quote) \
        with constraints to meet market maker obligations \
        (e.g., maintaining two-sided quotes). \n \
        Not usable with `OptionValue::DiscretionPriceType`.",
    ['m'] Midpoint 
        "Specifies a static (not pegged) midpoint order that executes \
        at the NBBO midpoint price at the time of order entry. \n \
        Not usable with `OptionValue::DiscretionPriceType`.",
}

impl Default for PriceType {
    fn default() -> Self {
        PriceType::Limit
    }
}

define_enum!{

    RejectReason 
        "Reason why an order or other request was rejected by NASDAQ.";

    [0x0001_u16] QuoteUnavailable,
    [0x0002_u16] DestinationClosed,
    [0x0003_u16] InvalidDisplay,
    [0x0004_u16] InvalidMaxFloor,
    [0x0005_u16] InvalidPegType,
    [0x0006_u16] FatFinger,
    [0x0007_u16] Halted,
    [0x0008_u16] IsoNotAllowed,
    [0x0009_u16] InvalidSide,
    [0x000A_u16] ProcessingError,
    [0x000B_u16] CancelPending,
    [0x000C_u16] FirmNotAuthorized,
    [0x000D_u16] InvalidMinQuantity,
    [0x000E_u16] NoClosingReferencePrice,
    [0x000F_u16] Other,

    [0x0010_u16] CancelNotAllowed,
    [0x0011_u16] PeggingNotAllowed,
    [0x0012_u16] CrossedMarket,
    [0x0013_u16] InvalidQuantity,
    [0x0014_u16] InvalidCrossOrder,
    [0x0015_u16] ReplaceNotAllowed,
    [0x0016_u16] RoutingNotAllowed,
    [0x0017_u16] InvalidSymbol,
    [0x0018_u16] Test,
    [0x0019_u16] LateLocTooAggressive,
    [0x001A_u16] RetailNotAllowed,
    [0x001B_u16] InvalidMidpoint,
    [0x001C_u16] InvalidDestination,
    [0x001D_u16] InvalidPrice,
    [0x001E_u16] SharesExceedThreshold,
    [0x001F_u16] MaximumNotionalValue,

    [0x0020_u16] RiskAggregateExposureExceeded,
    [0x0021_u16] RiskMarketImpact,
    [0x0022_u16] RiskRestrictedStock,
    [0x0023_u16] RiskShortSellRestricted,
    [0x0024_u16] RiskIsoNotAllowed,
    [0x0025_u16] RiskExceedsAdvLimit,
    [0x0026_u16] RiskFatFinger,
    [0x0027_u16] RiskLocateRequired,
    [0x0028_u16] RiskSymbolMessageRate,
    [0x0029_u16] RiskPortMessageRate,
    [0x002A_u16] RiskDuplicateMessageRate,
    [0x002B_u16] RiskShortSellNotAllowed,
    [0x002C_u16] RiskMarketOrderNotAllowed,
    [0x002D_u16] RiskPreMarketNotAllowed,
    [0x002E_u16] RiskPostMarketNotAllowed,
    [0x002F_u16] RiskShortSellExemptNotAllowed,

    [0x0030_u16] RiskSingleOrderNotionalExceeded,
    [0x0031_u16] RiskMaxQuantityExceeded,
    [0x0032_u16] ShoStateNotAvailable,
    [0x0033_u16] RiskIpoMarketBuyNotAllowed,
}

define_enum!{

    RestateReason: "Reason why an order was restated.";

    ['R'] Refresh "Refresh of display (on an order with reserves)",
    ['P'] Update "Update of displayed price",
}

define_enum!{

    Side: "Market side (i.e., buyer/seller)";

    ['B'] Buy,
    ['S'] Sell,
    ['T'] SellShort,
    ['E'] SellShortExempt 
        "Short sale order exempt from certain Regulation SHO restrictions, \
        such as the uptick rule or locate requirement.",
}

define_enum!{

    TimeInForce: "During which time span will the order be active on the book?";

    ['0'] Day 
        "Active during regular market hours on the day it is submitted.",
    ['3'] ImmediateOrCancel 
        "'IOC' \n \
        Must execute immediately, either fully or partially, \
        with any unfilled portion canceled. \
        It does not rest in the order book.",
    ['5'] GoodTilExtended 
        "'GTX' \n \
        Active during extended hours, including pre-market, \
        regular market hours, and after-hours. \n \
        If unexecuted by the end of extended hours, it is canceled.",
    ['6'] GoodTilTime 
        "'GTT' \n \
        Order that remains active until a user-defined expiration time, \
        to be provided via the `TagValue::ExpireTime` option. \n \
        NOTE: ExpireTime is required on orders with this TiF.",
    ['E'] AfterHours 
        "Only active after hours.",
}


