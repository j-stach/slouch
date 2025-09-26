
pub struct TagValue {
    /// The remaining length of the TagValue:
    /// Size of OptionValue + 1 (for option tag byte).
    /// Total length of TagValue can be calculated by adding 1 to this value.
    length_in_bytes: u8,
    option_value: OptionValue
}

impl TagValue {
    
    pub(crate) fn new(option_value: OptionValue) -> Self {

        use OptionValue::*;
        let length_in_bytes: u8 = match option_value {
            SecondaryOrdRefNum(..)  => 8,
            Firm(..)                => 4,
            MinQty(..)              => 4,
            CustomerType(..)        => 1,
            MaxFloor(..)            => 4,
            PriceType(..)           => 1,
            PegOffset(..)           => 4,
            DiscretionPrice(..)     => 8,
            DiscretionPriceType(..) => 1,
            DiscretionPegOffset(..) => 4,
            PostOnly(..)            => 1,
            RandomReserves(..)      => 4,
            Route(..)               => 4,
            ExpireTime(..)          => 4,
            TradeNow(..)            => 1,
            HandleInst(..)          => 1,
            BboWeightIndicator(..)  => 1,
            DisplayQuantity(..)     => 4,
            DisplayPrice(..)        => 8,
            GroupId(..)             => 2,
            SharesLocated(..)       => 1,
            LocateBroker(..)        => 4,
            Side(..)                => 1,
            UserRefIndex(..)        => 1,
        // NOTE: Add 1 byte to account for the option tag.
        } + 1;

        TagValue {
            length_in_bytes,
            option_value
        }
    }

    pub(crate) fn encode(&self) -> Vec<u8> {

        // TODO: 
        // get length for first byte
        // second byte is the OptionTag, match from OptionValue
        // subsequent bytes are the encoded OptionValue

        todo!{}
    }

    pub(crate) fn parse(data: Vec<u8>) -> Result<Self, OuchError> {

        // First byte is length_in_bytes
        let length_in_bytes = u8::from_be_bytes(data[0]);

        // TODO: 
        // Parse OptionValue based on OptionTag (second byte)

        todo!{}
    }
}

