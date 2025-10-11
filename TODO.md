
## TODO for crate:
- [ ] Comments & documentation (Copy from protocol PDF)


## Testing:
### requests
- [ ] Test handling of inappropriate options for requests
### options
- [ ] Test all options' encode & parse via applicable requests
- [x] multiple options
- [x] replacing option
- [ ] option order should not matter
- [ ] Errors for bad elements in quirky types (eg `DiscretionPriceType`)

### Integration testing
- [ ] [Schedule appointment](https://www.nasdaqtrader.com/Trader.aspx?id=TestingFacility)


## Future directions:
### Ease of use:
- [ ] Some field and type names may be shortened while remaining descriptive
- [ ] Defaults for request generators in alignment with protocol
- [ ] `Option` and `union`-based field access through request/response enums
- [ ] `add_options` for adding multiple tagvalues in a vec

