
# TODO for crate

### use `nsdq-util` to replace 
- [ ] helpers, 
- [x] enums and 
- [ ] string types, 
- [ ] ternary etc.

### messages
- [ ] Convert parsers to use `nom` (zero allocation)
- [x] `parse`, `encode` convention
- [ ] macro for defining request/response enums to avoid repetition

### options
- [x] `TagValue` parse more logically with `nom`
- [ ] Finish `TagValue` vars, `nsdq-util` needs `encode` for bool and ternary



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

