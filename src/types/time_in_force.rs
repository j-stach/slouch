
nsdq_util::define_enum!{

    TimeInForce: "During which time span will the order be active on the book?";

    ['0'] Day 
        "Active during regular market hours on the day it is submitted.",
    ['3'] ImmediateOrCancel 
        "'IOC' \n \
        Must execute immediately, either fully or partially, \
        with any unfilled portion canceled. \
        It does not rest in the order book.",
    ['5'] GoodTilExtended 
        "'GTX' \n \
        Active during extended hours, including pre-market, \
        regular market hours, and after-hours. \n \
        If unexecuted by the end of extended hours, it is canceled.",
    ['6'] GoodTilTime 
        "'GTT' \n \
        Order that remains active until a user-defined expiration time, \
        to be provided via the `TagValue::ExpireTime` option. \n \
        NOTE: ExpireTime is required on orders with this TiF.",
    ['E'] AfterHours 
        "Only active after hours.",
}

