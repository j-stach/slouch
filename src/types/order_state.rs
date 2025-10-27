
use nsdq_util::define_enum;

define_enum!{

    OrderState: 
        "Should be a boolean, but in the interest of maintaining consistency \
        with the wording in the spec, we use an enum.";

    ['L'] Live "Order is accepted and tracked in the system.",
    ['D'] Dead "Means that the order was accepted and automatically canceled.",
}

