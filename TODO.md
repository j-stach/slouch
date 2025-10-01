
## TODO for crate:
- [ ] Test OUCH compliance (via emulator API?)
- [ ] Unit testing
- [ ] Control `pub` for clean & safe API
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
- For `CancelOrder` request, does `quantity` > original do nothing?
- What if you have no account, or connection is rejected?
Will it simply timeout?

#### Defaults in message elements
- Do the defaults (e.g. blank) for optional fields apply when they are 
a non-optional part of a message? How best to handle these?
- [ ] Some enums have a "None" == b' ' that is not listed
- [ ] `Default` implemented for applicable fields


## TODO for library:
- [ ] Comments & documentation (Copy from protocol PDF)
- [ ] Double-check logic for messages and types

### Client:
- [x] Rewrite `OuchClient`
- [x] Increment `UserRefNum` 
- [x] Set `UserRefNum` based on `AccountQueryResponse`

### Macros:
- [x] Macros for easy message creation
```
#[macro_export]
macro_rules! my_struct {
    (field1: $f1:expr, field2: $f2:expr $(,)?) => {
        MyStruct::new($f1, $f2)
    };
}

my_struct!{ field1: 0u32, field2: true};
```

### Messages:
- [ ] TBD: change `options` method to get `&Vec<TagValue` directly?
- [ ] TBD: Use `assert` to enforce some errors at compile time?
- [ ] TBD: More efficient encoding than `Vec<u8>` heap allocation?

