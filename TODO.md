
## TODO for crate:
#### Defaults for options are not automatically stored/applied:

Add variants for types that need it
- [x] `CustomerType`: space = port default
- [x] `TagValue::TradeNow` needs enum instead of bool: space = port default
- [x] `HandleInst`: space = no instructions

Implement `Default` for types where it applies
- [x] `FirmId` - blank
- [x] `PriceType::Limit`

## Testing:
- [ ] Test `OuchClient` locally
- [ ] Test handling of inappropriate options for requests
- [ ] Errors for bad elements in quirky types (eg `DiscretionPriceType`)
- [ ] Parse `OuchResponse` with multiple options

### Integration testing
- [ ] [Schedule appointment](https://www.nasdaqtrader.com/Trader.aspx?id=TestingFacility)
- [ ] All msg & type enum variants should be tested during integration
- [ ] Test suite for stress and performance


## Future directions:
### Ease of use:
- [ ] Some field and type names may be shortened while remaining descriptive
- [ ] `Option`-based field access through request/response enums?
- [ ] `add_options` for adding multiple tagvalues in a vec

### Performance:
TBD. Plenty to be done.
- [ ] Convert parsers to use `nom` (zero allocation)?

