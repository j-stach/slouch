
use nsdq_util::define_enum;

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

