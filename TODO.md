
# slouch

# TODO for crate
- [ ] clean up imports

### messages
- [ ] Walk back `SystemEvent`, doesn't use Optional appendage
- [x] `add_option` for each request type



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
- [ ] `Option`-based field access through request/response enums?

### Performance:
TBD. Plenty to be done.

