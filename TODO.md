
# TODO for crate

### use `nsdq-util` to replace 
- [x] helpers, 
- [x] enums and 
- [x] string types, 
- [ ] `nsdq-util` bool & ternary, NaiveTime, etc. need both `parse` and `encode`
- [ ] import/re-export types eg `Mpid` and `StockSymbol`
- [ ] alias `as Price64` -> `as Price`

### messages
- [x] Convert parsers to use `nom` (zero allocation)
- [x] `parse`, `encode` convention
- [x] macro for defining request/response enums to avoid repetition
- [ ] Walk back `SystemEvent`, doesn't use Optional appendage

### options
- [x] `TagValue` parse more logically with `nom`
- [ ] Finish `TagValue` vars, needs `encode` for bool and ternary



## Testing:
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

