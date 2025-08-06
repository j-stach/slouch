
## TODO for crate:
- [ ] Test OUCH compliance (via emulator API?)
- [ ] Unit testing
- [ ] `README.md` minimal examples

### Readme examples:
- [ ] `OrderToken` creation
- [ ] `OuchClient` walkthrough
- [ ] Message send and receive using `OuchIn` and `OuchOut` via `OuchClient`

### For library:
- [x] Convert message types to use `OrderToken`
- [x] `OrderToken` encode method
- [ ] Convert functions to use `OuchError` instead of `unwrap` 
- [x] Break up messages into submodules for structs
- [ ] Documentation for message types
- [ ] Types for `FirmId` and `Symbol` (stock)
- [ ] Use `std::net::SocketAddr` type for `OuchClient`


