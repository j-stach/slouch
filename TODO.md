
# nsdq-util
- [ ] `nsdq-util` bool & ternary, NaiveTime, etc. need both `parse` and `encode`
- [ ] change `define_enum` to create array instead of tuple
- [ ] `define_enum` needs to handle `u16` tags better

# TODO for crate
- [ ] Replace bad element error with option?

### types
- [x] condense type files into `strings.rs` and `enums.rs`
- [x] import/re-export types eg `Mpid` and `StockSymbol`
- [x] alias `as Price64` -> `as Price`

### messages
- [ ] Walk back `SystemEvent`, doesn't use Optional appendage

### options
- [ ] Finish `TagValue` vars, needs `encode` for bool and ternary
- [ ] `add_options` for adding multiple tagvalues in a vec



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

