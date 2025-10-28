
use nsdq_util::NaiveTime;
use crate::types::EventCode;

crate::msg::define_msg!{
    SystemEvent: "";
        //timestamp: NaiveTime,
        //{parse, encode}
        event_code: EventCode
        { EventCode::parse, EventCode::encode }
}

