
## Dev questions
Some messages do not need to encode optional appendage:
- if there are no options? can it be present with length 0?
- are there any "mandatory" optional fields for enter and replace that would make the appendage always necessary?
- is the length marker always present for responses?
- [ ] `Option<OptionalAppendage>`
- "You will receive one and only one of these two for each replacement." `OrderReplaced`

## TODO for crate:
### Comments & documentation (Copy from protocol PDF)
- [x] Request
- [x] Response
- [ ] Optional Fields
- [ ] Types
- [ ] Client


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
- [ ] Some field and type names may be shortened while remaining descriptive
- [ ] Defaults for request generators in alignment with protocol
- [ ] `Option`-based field access through request/response enums?
- [ ] `add_options` for adding multiple tagvalues in a vec
- [ ] More set-like impl of `PartialEq` for `OptionalAppendage`
- [ ] Alpha-like constructors renamed to `from`
- [ ] `try_from` and `to_bytes` for responses and requests, for consistency

