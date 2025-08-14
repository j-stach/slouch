
## TODO for crate:
- [ ] Test OUCH compliance (via emulator API?)
- [ ] Unit testing
- [ ] `README.md` minimal examples

### Readme examples:
- [ ] `OrderToken` creation
- [ ] `OuchClient` walkthrough
- [ ] Message send and receive using `OuchIn` and `OuchOut` via `OuchClient`

## TODO for library:
### For messages:
- [ ] Convert functions to use `OuchError` instead of `unwrap` 
- [ ] Documentation for message types
- [ ] Ensure proper encoding of numeric values for protocol messages

### For types:
- [x] Ensure `FirmId` has fixed length 4
- [x] Move `OrderToken`, `FirmId`, `StockSymbol` into `types` module
- [ ] Strongly-typed enums for all `char` options in messages
- [ ] Conversion for `u32` to timestamp/duration

### For client:
- [ ] Use `std::net::SocketAddr` type for `OuchClient`
- [ ] Finish send/recv for `OuchClient`
- [ ] `OrderTokenGenerator` and `OrderTokenEncoding` for use in `OuchClient`
- [ ] Rollover for token sequentialization based on `OrderTokenEncoding`


