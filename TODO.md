
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
- [ ] More messages for NASDAQ's OUCH 5.0
- [ ] Double-check protocol offsets for 5.0

### For types:
- [ ] Strongly-typed enums for all `char` options in messages
- [x] Conversion for `u64` to timestamp/duration
- [ ] Types for shares and price to enforce bounds
- [ ] Bring type names into alignment with OUCH

### For client:
- [ ] Finish send/recv for `OuchClient`


