
// TODO: Describes the option tag length during encode/parse
// TODO: Holds the value of the option

/// These names are kept as similar as possible to the corresponding options
/// as documented in the OUCH 5.0 specifications.
#[derive(Debug, Serialize, Deserialize)]
pub enum OptionValue {
    // TODO: Each of these gets a type
    SecondaryOrdRefNum(OrdRefNum),
    Firm(FirmId),
    MinQty,
    CustomerType(CustomerType),
    MaxFloor,
    PriceType(PriceType),
    PegOffset,
    DiscretionPrice(Price),
    DiscretionPriceType(PriceType),
    DiscretionPegOffset,
    PostOnly,
    RandomReserves,
    Route,
    ExpireTime,
    TradeNow,
    HandleInst,
    BboWeightIndicator,
    DisplayQuantity,
    DisplayPrice,
    GroupId,
    SharesLocated,
    LocateBroker,
    Side,
    UserRefIdx
}

impl OptionValue {

    pub(crate) fn encode(&self) -> Vec<u8> {

        // TODO:

        todo!{}
    }

    pub(crate) fn parse(data: Vec<u8>) -> Result<Self, OuchError> {

        // TODO: 

        todo!{}
    }
}
