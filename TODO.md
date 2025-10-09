
## TODO for crate:
- [ ] Comments & documentation (Copy from protocol PDF)


## Testing:

### Unit testing

#### response
- [x] event
- [x] broken
- [x] query
- [x] accepted
- [x] executed
- [x] modified
- [x] permission
- [x] rejected
- [x] replaced
- [x] updated
- [x] canceled

#### options
- [ ] Test all options' encodings to applicable requests
- [ ] Test handling of inappropriate options for requests

### Integration testing
- [ ] [Schedule appointment](https://www.nasdaqtrader.com/Trader.aspx?id=TestingFacility)


## Future directions:
### Ease of use:
- [ ] Some field and type names may be shortened while remaining descriptive
- [ ] Defaults for request generators in alignment with protocol
- [ ] `Option` and `union`-based field access through request/response enums

