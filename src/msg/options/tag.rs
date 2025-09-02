
pub struct TagValue {
    /// The length of the remaining OptionValue.
    length_in_bytes: u8,
    optional_field: OptionValue
}

impl TagValue {
    
    pub(crate) fn new(option_value: OptionValue) -> Self {

        // TODO: Get the length of the OptionValue and set the length_in_bytes

        todo!{}
    }

    pub(crate) fn encode(&self) -> Vec<u8> {

        // TODO: 
        // get length for first byte
        // second byte is the OptionTag, match from OptionValue
        // subsequent bytes are the encoded OptionValue

        todo!{}
    }

    pub(crate) fn parse(data: Vec<u8>) -> Result<Self, OuchError> {

        // TODO: Parse OptionValue based on OptionTag (second byte)

        todo!{}
    }
}

