
use crate::error::OuchError;
use super::TagValue;


/// Contains optional fields that may be included in a Request/Response.
/// Only one instance of each variant of `TagValue` is allowed --
/// if another is added, the old one will be overwritten.
#[derive(Debug, Clone, Eq)]
pub struct OptionalAppendage {
    tag_values: Vec<TagValue>
}

impl OptionalAppendage {

    pub(crate) fn new() -> Self {
        OptionalAppendage {
            tag_values: Vec::new()
        }
    }

    // Read the options in the optional appendage.
    pub(crate) fn tag_values(&self) -> &Vec<TagValue> {
        &self.tag_values
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
            appendage_length += etv.len() as u16 
        }

        // Build the final array, starting with the appendage length.
        let al_be = appendage_length.to_be_bytes();
        let mut encoded_appendage = vec![al_be[0], al_be[1]];
        let payload = encoded_tag_values
            .into_iter()
            .flatten()
            .collect::<Vec<u8>>();
        encoded_appendage.extend(payload);

        encoded_appendage
    }

    // For `ModifyOrder` and `AccountQuery` requests, 
    // the Optional Appendage (including length marker) is itself optional.
    pub(crate) fn encode_nothing_if_empty(&self) -> Vec<u8> {

        if self.tag_values.len() == 0 {
            vec![]
        } else {
            self.encode()
        }
    }

    pub(crate) fn parse(mut input: &[u8]) -> nom::IResult<&[u8], Self> {

        let mut tag_values: Vec<TagValue> = Vec::new();

        // Ignore the first two bytes, they will contain the appendage length.
        let mut tag_start = 2u8;

        // Repeat this until the array is parsed:
        while tag_start < input.len() as u8 {
            // The first byte in the tag describes the length.
            if let Some(tag_len) = input.get(tag_start as usize) {

                // Use it to get the next X bytes (this one not included).
                // Assumes the length of a tag is always a u8.
                let tag_end = tag_start + *tag_len;
                if tag_end < input.len() as u8 {

                    let raw_tag_value = &input[
                        ((tag_start as usize) + 1)..=(tag_end as usize)
                    ];

                    let (ipt, tag_value) = TagValue::parse(raw_tag_value)?;
                    input = ipt;
                    tag_values.push(tag_value);

                    // The byte after (i.e., X + 1) will contain the length 
                    // of the next tag value (if there is one).
                    tag_start = tag_end as u8 + 1;

                } else {
                    return Err(
                        nom::Err::Error(nom::error::Error::new(
                            input, 
                            nom::error::ErrorKind::Tag
                        ))
                    )
                }
            }
        }

        Ok((input, Self{ tag_values }))
    }
}

/// Uses a more set-like approach than can be derived for Vec.
impl PartialEq for OptionalAppendage {
    fn eq(&self, other: &OptionalAppendage) -> bool {

        let s_tvs = self.tag_values();
        let o_tvs = other.tag_values();

        // There should be no more and no fewer elements in the other.
        if s_tvs.len() != o_tvs.len() { return false }

        // For every element in self, 
        // there should be an identical element somewhere in other.
        for stv in s_tvs.iter() {
            if let None = o_tvs.iter().find(|otv| *otv == stv) {
                return false
            }
        }

        true
    }
}
