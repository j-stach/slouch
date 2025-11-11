
use crate::types::{ NaiveTime, EventCode };

// TODO: Implement manually, does not use optional appendage

crate::msg::define_msg!{
    SystemEvent: "";
        //timestamp: NaiveTime,
        //{parse, encode}
        event_code: EventCode
        { EventCode::parse, EventCode::encode }
}

