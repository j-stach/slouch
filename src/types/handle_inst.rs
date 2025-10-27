
use nsdq_util::define_enum;

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

