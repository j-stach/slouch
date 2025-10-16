
## TODO for crate:
- [x] Change `OuchClient` to wrap TCP connection, login/heartbeat separately
- [ ] `UnknownResponse` for non-order-related response messages
- [ ] Update readme for limited scope

## Testing:
- [ ] Clean up tests? (eg, dry)
- [ ] Test handling of inappropriate options for requests
- [ ] Errors for bad elements in quirky types (eg `DiscretionPriceType`)
- [ ] Parse `OuchResponse` with multiple options

### Integration testing
- [ ] [Schedule appointment](https://www.nasdaqtrader.com/Trader.aspx?id=TestingFacility)
- [ ] All msg & type enum variants should be tested during integration


## Future directions:
### Ease of use:
- [ ] Some field and type names may be shortened while remaining descriptive
- [ ] Defaults for request generators in alignment with protocol
- [ ] `Option`-based field access through request/response enums?
- [ ] `add_options` for adding multiple tagvalues in a vec

### Performance:
TBD. Plenty to be done.

