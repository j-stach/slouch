
use nsdq_util::define_enum;

define_enum!{

    RestateReason: "Reason why an order was restated.";

    ['R'] Refresh "Refresh of display (on an order with reserves)",
    ['P'] Update "Update of displayed price",
}

