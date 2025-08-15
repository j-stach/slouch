
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
- [ ] Documentation for message types
- [x] Helper function for encoding big-endian numbers (shares, etc.)

### For types:
- [ ] Strongly-typed enums for all `char` options in messages
- [ ] Conversion for `u64` to timestamp/duration

### For client:
- [ ] Use `std::net::SocketAddr` type for `OuchClient`
- [ ] Finish send/recv for `OuchClient`
- [ ] `OrderTokenGenerator` and `OrderTokenEncoding` for use in `OuchClient`
- [ ] Rollover for token sequentialization based on `OrderTokenEncoding`


