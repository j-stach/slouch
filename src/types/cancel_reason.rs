
use nsdq_util::define_enum;

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

