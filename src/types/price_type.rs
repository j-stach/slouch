
use nsdq_util::define_enum;

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

