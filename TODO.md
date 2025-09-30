
## TODO for crate:
- [ ] Test OUCH compliance (via emulator API?)
- [ ] Unit testing
- [ ] Control `pub` for clean API
- [ ] `README.md` with minimal examples
- [ ] `async` and `log` features

### Contact dev team for advice on odd types:
- `RouteId`, `BrokerId`: Any restrictions?
- `SignedPrice`: Any bounds?
- `ExpireTime` must be "<" or "<=" 86400?
- Some enums (e.g., `LiquidityFlag`) contain numbers despite labeled as "Alpha"
Is this an exception, or are there other cases where this may apply?
- Is "m = midpoint" a variant for `PriceType`?
- `CustomerType` is not accpeted on `ReplaceOrder`, and `Side` is, 
but these are not addressed in the prose of the note.

#### Defaults in message elements
- Do the defaults (e.g. blank) for optional fields apply when they are 
a non-optional part of a message? How best to handle these?
- [ ] Some enums have a "None" == b' ' that is not listed
- [ ] `Default` implemented for applicable fields


## TODO for library:
- [ ] Debug compilation errors

### Client:
- [ ] Finish send/recv for `OuchClient`
- [ ] Update client to increment `UserRefNum` instead of generating tokens


## TODO for Messages
- [ ] Comments & documentation (Copy from protocol PDF)
- [x] `options` methods for messages
- [x] Check each message when adding options to ensure types accepted
- [ ] `OptionalAppendage` needs to overwrite like-variants where found

### Macros:
- [ ] Macros for easy type and message creation
```
macro_rules! my_struct {
    (field1: $f1:expr, field2: $f2:expr $(,)?) => {
        MyStruct::new($f1, $f2)
    };
}

my_struct!{ field1: 0u32, field2: true};
```


