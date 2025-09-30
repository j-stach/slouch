
use super::TagValue;


/// Contains optional fields that may be included in a Request/Response.
/// Only one instance of each variant of `TagValue` is allowed --
/// if another is added, the old one will be overwritten.
#[derive(Debug, Serialize, Deserialize)]
pub struct OptionalAppendage {
    tag_values: Vec<TagValue>
}

impl OptionalAppendage {

    pub(crate) fn new() -> Self {
        OptionalAppendage {
            tag_values: Vec::new()
        }
    }

    // NOTE: Does not check for validity of an option for each message type:
    // filter valid options within the message's `add_option` method.
    pub(crate) fn add(&mut self, tag_value: TagValue) {

        let tvs = &mut self.tag_values;

        use std::mem::discriminant as dd;
        if let Some(i) = tvs.iter().position(|tv| dd(tv) == dd(&tag_value)) {
            tvs[i] = tag_value
        } else {
            tvs.push(tag_value)
        }

    }

    pub(crate) fn encode(&self) -> Vec<u8> {

        // Map vector of tag values to a vector of encoded byte arrays.
        let encoded_tag_values: Vec<Vec<u8>> = self.tag_values.iter()
            .map(|tag_value| { tag_value.encode() })
            .collect();
        
        // Sum the lengths of the encoded tags to get appendage length.
        let mut appendage_length = 0u16;
        for etv in encoded_tag_values.iter() { 
            appendage_length += etv.len() 
        }

        // Build the final array, starting with the appendage length.
        let mut encoded_appendage = vec![appendage_length.to_be_bytes()];
        encoded_appendage.extend(encoded_tag_values.flatten());

        encoded_appendage
    }

    pub(crate) fn parse(data: Vec<u8>) -> Result<Self, OuchError> {

        let mut tag_values: Vec<TagValue> = Vec::new();

        // Ignore the first two bytes, they will contain the appendage length.
        let mut tag_start = 2;

        // Repeat this until the array is parsed:
        while tag_start < data.len() {
            // The first byte in the tag describes the length.
            if let Some(tag_len) = data.get(tag_start) {
                // Use it to get the next X bytes (this one not included).
                let tag_end = tag_start + tag_len;
                if tag_end < data.len() {
                    let raw_tag_value = &data[tag_start + 1..=tag_end];
                    let tag_value = TagValue::parse(raw_tag_value)?;
                    tag_values.push(tag_value);
                    // The byte after (i.e., X + 1) will contain the length 
                    // of the next tag value (if there is one).
                    tag_start = tag_end + 1;
                } else {
                    return Err(
                        OuchError::Parse("OptionalAppendage".to_string())
                    )
                }
            }
        }

        Ok(Self{ tag_values })
    }
}

