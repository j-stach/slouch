
use crate::types::{ NaiveTime, EventCode };

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SystemEvent {
    timestamp: NaiveTime,
    event_code: EventCode
}

#[allow(dead_code)]
impl SystemEvent {

    // Data contains package without type tag, 
    // so all offsets should be one less than those in the spec.
    pub(crate) fn parse(input: &[u8]) -> nom::IResult<&[u8], Self> {

        let (input, timestamp) = nsdq_util::parse_ouch_time_bold(input)?;
        let (input, event_code) = EventCode::parse(input)?;

        Ok((input, Self { timestamp, event_code }))
    }

    pub(crate) fn encode(&self) -> Vec<u8> {
        
        let mut bytes: Vec<u8> = Vec::with_capacity(9usize);
        bytes.extend(self.event_code.encode());

        bytes
    }

    pub fn timestamp(&self) -> NaiveTime {
        self.timestamp
    }


    pub fn event_code(&self) -> EventCode {
        self.event_code
    }
}

