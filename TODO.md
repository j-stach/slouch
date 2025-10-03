
## TODO for crate:
- [ ] Update `cargo.toml` with necessary info
- [ ] Comments & documentation (Copy from protocol PDF)
- [ ] `README.md` with minimal examples

## Fix in library:
- [ ] `OrderToken` needs to accept digits and spaces

## Testing:
### Unit testing
`test` module within `src` 
- Double-check logic for messages and types
- Ensure types & bounds are enforced in edge cases
- [ ] Macros and helper functions for quickly rewriting similar tests

#### request
- [ ] cancel
- [ ] modify
- [ ] permission
- [ ] enter
- [ ] replace

#### response
- [ ] Test response parsing and field access

#### options
- [ ] Test all options' encodings to applicable requests
- [ ] Test handling of inappropriate options for requests

### Integration testing
- [ ] [Schedule appointment](https://www.nasdaqtrader.com/Trader.aspx?id=TestingFacility)

