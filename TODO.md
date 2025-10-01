
## TODO for crate:
- [ ] Test OUCH compliance (via emulated API?)
- [ ] Unit testing
- [ ] `README.md` with minimal examples

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
- What if you have no account, or connection is rejected by server? 
Will it simply timeout?

#### Defaults in message elements
- Do the defaults (e.g. blank) for optional fields apply when they are 
a non-optional part of a message? How best to handle these?
- Some enums have a "None" == b' ' that is not listed
- [ ] `Default` implemented for applicable fields, if necessary


## TODO for library:
- [ ] Comments & documentation (Copy from protocol PDF)
- [ ] Double-check logic for messages and types
- [ ] `async` and `log` features for client
- [ ] Use `assert!` to enforce macro errors at compile time?
- [ ] Derive `Copy` for enums
- [ ] Change `user_ref_num` function to get `UserRefNum` instead of `u32`

### Messages:
- [ ] TBD: change `options` method to get `&Vec<TagValue>` directly?
- [ ] TBD: More efficient encoding than `Vec<u8>` heap allocation? 
Don't let `TagValue` dictate `encode`, it can convert to `Vec` as it needs to.
- [ ] `Copy` for fixed length string types

