
# slouch

# TODO for crate
- [ ] Replace bad element error with option?
- [ ] enums completed

### messages
- [ ] Walk back `SystemEvent`, doesn't use Optional appendage

### options
- [x] Finish `TagValue` vars, needs `encode` for bool and ternary
- [ ] `add_options` for adding multiple tagvalues in a vec
- [x] `TagValue` must produce `nom` error when fails to parse



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
- [x] Some field and type names may be shortened while remaining descriptive
- [ ] `Option`-based field access through request/response enums?

### Performance:
TBD. Plenty to be done.

