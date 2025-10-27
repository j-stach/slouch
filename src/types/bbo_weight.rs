
use nsdq_util::define_enum;

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

