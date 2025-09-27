
## TODO for crate:
- [ ] Contact dev team for advice on odd types (such as `SignedPrice`)
- [ ] Test OUCH compliance (via emulator API?)
- [ ] Unit testing
- [ ] Control `pub` for clean API
- [ ] `README.md` minimal examples
- [ ] `async` and `log` features
- [ ] Add citation to dev resources for [AIQ Strategy values](https://www.nasdaqtrader.com/TraderNews.aspx?id=ETU2023-8)


## TODO for library:
- [ ] Debug helpers
- [ ] `i32_from_be_bytes` and `u8_from_be_bytes` helpers
- [ ] `BadElementError::InvalidEnum` needs to accept non-`char` values  

### For client:
- [ ] Finish send/recv for `OuchClient`

## TODO for Messages
- [ ] Comments & documentation (Copy from protocol PDF)
- [ ] Macros for easy type and message creation
- [ ] `options` methods for messages
- [ ] Check each message when adding options to ensure types accepted

### Strong types fixed
- [ ] Double-check compliance for type quirks
- [ ] `Alpha` type and `ElapsedTime` type for compliance
- [ ] Some enums have a "None" == b' ' that is not listed
- [ ] `Default` implemented for applicable fields
- [ ] `FirmId` check for capitalization
- [ ] `UserRefNum` begins at 1, not 0
- [x] `SignedPrice`
- [x] `RouteId`
- [x] `BrokerId`


