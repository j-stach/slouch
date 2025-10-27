
use nsdq_util::define_enum;

define_enum!{

    CrossType: 
        "Designates the specific market session, auction, \
        or execution mechanism the order should participate in, \
        influencing when and how it executes.";

    ['N'] ContinuousMarket 
        "Standard limit order for ongoing matching in the regular trading book.\
        (NASDAQ Only)",
    ['O'] Opening 
        "Participates in auction to determine the official opening price. \
        (NASDAQ Only)",
    ['C'] Closing 
        "Joins the closing auction for the official close. (NASDAQ Only)",
    ['H'] Halt 
        "Reopenings after halts or initial public offerings. (NASDAQ Only)",
    ['S'] Supplemental 
        "Provides additional liquidity during crosses. (NASDAQ Only)",
    ['R'] Retail 
        "Specific to BX exchange for retail-designated orders. (BX Only)",
    ['E'] ExtendedLife 
        "For orders with longer persistence. (NASDAQ Only)",
    ['A'] AfterHoursClose 
        "For after-hours closing mechanisms. (NASDAQ Only)",
}

