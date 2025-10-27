
use nsdq_util::define_enum;

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

