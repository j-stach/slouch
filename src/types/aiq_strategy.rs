
use nsdq_util::define_enum;

define_enum!{

    AiqStrategy: 
        "I am trusting the user to know what these mean. \
        If you do not know, then chances are you will not need them. \
        https://www.nasdaqtrader.com/TraderNews.aspx?id=ETU2023-8";

    ['O'] CancelOldestMpid,
    ['W'] CancelNewestMpid,
    ['D'] DecrementBothMpid,
    ['Y'] DecrementBothNoDetailsMpid,
    ['o'] CancelOldestOrgId,
    ['w'] CancelNewestOrgId,
    ['d'] DecrementBothOrgId,
    ['y'] DecrementBothNoDetailsOrgId,
}

