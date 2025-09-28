
## TODO for crate:
- [ ] Test OUCH compliance (via emulator API?)
- [ ] Unit testing
- [ ] Control `pub` for clean API
- [ ] `README.md` with minimal examples
- [ ] `async` and `log` features

### Contact dev team for advice on odd types:
- [ ] `RouteId`, `BrokerId`: Any restrictions?
- [ ] `SignedPrice`: Any bounds?


## TODO for library:
- [x] `i32_from_be_bytes` helpers
- [ ] `BadElementError::InvalidEnum` needs to accept non-`char` values  

### Client:
- [ ] Finish send/recv for `OuchClient`
- [ ] Update client to increment `UserRefNum` instead of generating tokens


## TODO for Messages
- [ ] Comments & documentation (Copy from protocol PDF)
- [ ] `options` methods for messages
- [ ] Check each message when adding options to ensure types accepted

### Macros:
- [ ] Macros for easy type and message creation
```
macro_rules! my_struct {
    (field1: $f1:expr, field2: $f2:expr) => {
        MyStruct::new($f1, $f2)
    };
}

my_struct!{ field1: 0u32, field2: true};
```

### Strong types fixed
Double-check compliance for type quirks:
- [ ] Some enums have a "None" == b' ' that is not listed
- [ ] `Default` implemented for applicable fields
- [ ] `ElapsedTime` type

