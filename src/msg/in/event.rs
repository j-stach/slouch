
use serde::{ Deserialize, Serialize };
use crate::error::OuchError;

#[derive(Debug, Serialize, Deserialize)]
pub struct SystemEvent {
    pub event_code: char,
}

impl SystemEvent {

    // TODO Errors
    pub(super) fn parse(data: &[u8]) -> Result<SystemEvent, OuchError> {

        if data.len() < 1 {
            return Err("SystemEvent: insufficient data".into());
        }
        
        Ok(SystemEvent {
            event_code: data[0] as char,
        })
    }

}

