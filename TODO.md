
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
- [ ] `BadElementError::InvalidEnum` needs to accept non-`char` values  

### For client:
- [ ] Finish send/recv for `OuchClient`

## TODO for Messages
- [ ] Comments & documentation (Copy from protocol PDF)
- [ ] `FirmId` check for capitalization
- [ ] Macros for easy type and message creation
- [ ] Some enums have a "None" == b' ' that is not listed
- [ ] `Default` implemented for applicable fields
- [ ] `options` methods for messages
- [ ] Check each message when adding options to ensure types accepted

### For `OptionalAppendage`
#### Options need types 
- [x] `TagValue` and `OptionValue` combined as a single enum
- [x] `OptionalAppendage` must slice raw byte array using length-mark bytes 
#### Strong types for optional fields (there are many)
- [ ] `SignedPrice`
- [ ] `Route`
- [ ] `Broker`


