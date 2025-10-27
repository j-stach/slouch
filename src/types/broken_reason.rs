
use nsdq_util::define_enum;

define_enum!{

    BrokenReason: "Reason a trade execution was broken.";

    ['E'] Erroneous "Trade deemed 'clearly erroneous.'",
    ['C'] Consent "Both parties agreed to break the trade.",
    ['S'] Supervisory "Manually broken by NASDAQ supervisory terminal.",
    ['X'] External "Broken by external third party.",
}

