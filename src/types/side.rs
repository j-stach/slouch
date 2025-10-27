
use nsdq_util::define_enum;

define_enum!{

    Side: "Market side (i.e., buyer/seller)";

    ['B'] Buy,
    ['S'] Sell,
    ['T'] SellShort,
    ['E'] SellShortExempt 
        "Short sale order exempt from certain Regulation SHO restrictions, \
        such as the uptick rule or locate requirement.",
}

