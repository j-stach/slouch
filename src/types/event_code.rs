
use nsdq_util::define_enum;

define_enum!{
    
    EventCode: "Identifier for a system-wide signal.";

    ['S'] StartOfDay 
        "Indicates that exchange is open and is ready to accept orders.",
    ['E'] EndOfDay 
        "Exchange is closed and will no longer accept new orders. \n \
        NOTE: Still possible to receive breaks and cancels after this event.",
}

