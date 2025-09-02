
mod value;
mod tag;

pub use {
    option::OptionValue,
    tag::TagValue
}

// TODO
// TBD: Use Option<Type> as a field where options are relevant?
// Then use this type as an intermediary to encode it
// OR
// Use Type directly or via Optional trait in a builder pattern to add options

/// Contains optional fields included in a Request/Response.
#[derive(Debug, Serialize, Deserialize)]
pub(super) struct OptionalAppendage {
    // This tracks the total length of the optional fields as they are added.
    length_in_bytes: u16,
    tag_values: Vec<TagValue>
}

impl OptionalAppendage {

    pub(crate) fn new() -> Self {
        OptionalAppendage {
            length_in_bytes: 0u16,
            tag_values: Vec::new()
        }
    }

    pub(crate) fn add(option_value: OptionValue) {
        let tag_value = TagValue::new(option_value);

        // TODO: Check bounds before increasing
        // NOTE: Add 1 to account for `u8` length_in_bytes field in the TagValue.
        self.length_in_bytes += tag_value.length_in_bytes + 1;
        self.tag_values.push(tag_value)
    }

    // TODO:
    pub(crate) fn encode(&self) -> Vec<u8> {
        todo!{}
    }

    // TODO:
    pub(crate) fn parse(data: Vec<u8>) -> Result<Self, OuchError> {
        todo!{}
    }
}

