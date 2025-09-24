
use chrono::NaiveTime;

use serde::{ Deserialize, Serialize };
use crate::{
    error::OuchError,
    types::EventCode,
    helper::{
        nanosec_from_midnight,
        u64_from_be_bytes,
    },
};


///
#[derive(Debug, Serialize, Deserialize)]
pub struct SystemEvent {
    pub timestamp: NaiveTime,
    pub event_code: EventCode,
}

impl SystemEvent {

    pub(super) fn parse(data: &[u8]) -> Result<SystemEvent, OuchError> {

        if data.len() != 9 {
            return Err(BadElementError::WrongSize(
                "SystemEvent".to_string(), 9, data.len()
            ).into())
        }

        let ts = u64_from_be_bytes(&data[0..=7])?;
        let timestamp = nanosec_from_midnight(ts);

        let ec = &data[8];
        let event_code = EventCode::parse(ec)?;

        Ok(SystemEvent { timstamp, event_code })
    }

}

