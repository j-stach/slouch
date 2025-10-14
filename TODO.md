
## TODO for crate:
- [ ] Some field and type names may be shortened while remaining descriptive
### msg
- [ ] `Option<OptionalAppendage>`
- [ ] `add_options` for adding multiple tagvalues in a vec
### options
- [x] More set-like impl of `PartialEq` for `OptionalAppendage`
### types
- [ ] Default values for types eg `CustomerType`

## Dev questions
Some messages do not need to encode optional appendage:
- if there are no options? can it be present with length 0?
- are there any "mandatory" optional fields for enter and replace that would make the appendage always necessary?
- "You will receive one and only one of these two for each replacement." `OrderReplaced`

## Testing:
- [ ] Clean up tests (eg, dry)
- [ ] Test handling of inappropriate options for requests
- [ ] Errors for bad elements in quirky types (eg `DiscretionPriceType`)
- [ ] Parse `OuchResponse` with multiple options

### Integration testing
- [ ] [Schedule appointment](https://www.nasdaqtrader.com/Trader.aspx?id=TestingFacility)
- [ ] All msg & type enum variants can be tested during integration


## Future directions:
### Ease of use:
- [ ] Defaults for request generators in alignment with protocol
- [ ] `Option`-based field access through request/response enums?

