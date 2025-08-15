
## TODO for crate:
- [ ] Test OUCH compliance (via emulator API?)
- [ ] Unit testing
- [ ] `README.md` minimal examples
- [ ] `async` and `log` features

### Readme examples:
- [ ] `OrderToken` creation
- [ ] `OuchClient` walkthrough
- [ ] Message send and receive using `OuchIn` and `OuchOut` via `OuchClient`


## TODO for library:
### For messages:
- [ ] Documentation for message types

### For types:
- [ ] Strongly-typed enums for all `char` options in messages
- [ ] Conversion for `u64` to timestamp/duration

### For client:
- [x] Use `std::net::SocketAddr` type for `OuchClient`
- [ ] Finish send/recv for `OuchClient`
- [x] `OrderTokenGenerator` and `OrderTokenEncoding` for use in `OuchClient`
- [x] Rollover for token sequentialization based on `OrderTokenEncoding`


