
/// Contains types for messages accepted by NASDAQ.
//mod request;
//pub use request::*;

/// Contains types for responses from NASDAQ.
//mod response;
//pub use response::*;

/// Contains types for optional message appendages.
mod options;
use options::OptionalAppendage;
pub use options::TagValue;


macro_rules! define_msg {
    (
        [$tag:expr] $msg_name:ident: $($msg_doc:expr;)?
            $(
                $field_name:ident: $field_type:ident { 
                    $field_parser:expr,
                    $field_encoder:expr
                }
            ),* $(,)?
    ) => {

        #[derive(Debug, Clone)]
        $(#[doc = $msg_doc])?
        pub struct $msg_name {
            $(
                $field_name: $field_type,
            )*
            //optional_appendage: OptionalAppendage
        }

        impl $msg_name {

            // Data contains package without type tag, 
            // so all offsets should be one less than those in the spec.
            pub fn parse(input: &[u8]) -> nom::IResult<&[u8], Self> {

                $(
                    let (input, $field_name): (&[u8], $field_type) 
                        = $field_parser(input)?;
                )*
                //let (input, opts) = OptionalAppendage::parse(input)?;


                Ok((input, Self {
                    $( $field_name, )*
                    //optional_appendage: opts
                }))
            }

            pub fn encode(&self) -> Vec<u8> {
                
                let mut bytes: Vec<u8> = Vec::with_capacity(1024usize);

                bytes.push($tag);
                $( bytes.extend(self.$field_name.$field_encoder()); )*
                //bytes.extend(
                //self.optional_appendage.encode_nothing_if_empty());

                bytes
            }


            $(
                pub fn $field_name(&self) -> $field_type { self.$field_name }
            )*

            ///// Get read-only access to the message's optional fields.
            //pub fn options(&self) -> &Vec<TagValue> {
            //    &self.optional_appendage.tag_values()
            //}

        }
    }
}

