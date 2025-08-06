
## TODO for crate:
- [ ] Test OUCH compliance (via emulator API?)
- [ ] Unit testing
- [ ] `README.md` minimal examples

### Readme examples:
- [ ] `OrderToken` creation
- [ ] `OuchClient` walkthrough
- [ ] Message send and receive using `OuchIn` and `OuchOut` via `OuchClient`

### For library:
- [ ] Documentation for message types
- [ ] Convert functions to use `OuchError` instead of `unwrap` 
- [x] Types for `FirmId` and `StockSymbol` 
- [x] Convert messages to use `FirmId` and `StockSymbol`
- [ ] Strongly-typed enums for all `char` options and variants?
- [ ] Conversion for `u32` to timestamp/duration
- [ ] Use `std::net::SocketAddr` type for `OuchClient`
- [ ] Finish send/recv for `OuchClient`

