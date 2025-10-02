
## TODO for crate:
- [ ] Update `cargo.toml` with necessary info
- [ ] Comments & documentation (Copy from protocol PDF)
- [ ] `README.md` with minimal examples

### Contact dev team for advice on odd types:
- `RouteId`, `BrokerId`: Any restrictions?
- `SignedPrice`: Any bounds?
- `ExpireTime` must be "<" or "<=" 86400?
- Some enums (e.g., `LiquidityFlag`) contain numbers despite labeled as "Alpha"
Is this an exception, or are there other cases where this may apply?
- Is "m = midpoint" a variant for `PriceType`?
- `CustomerType` is not accepted on `ReplaceOrder`, and `Side` is, 
but these are not addressed in the prose of the note.
- For `CancelOrder` request, does `quantity` > original do nothing?
- What if you have no account, or connection is rejected by server? 
Will it simply timeout?

#### Defaults in message elements
- Do the defaults (e.g. blank) for optional fields apply when they are 
a non-optional part of a message? How best to handle these?
- Some enums have a "None" == b' ' that is not listed
- [ ] `Default` implemented for applicable fields, if necessary


## Testing:
### Unit testing
`test` module within `src` 
- Double-check logic for messages and types
- Ensure types & bounds are enforced in edge cases
- [x] helpers
- [ ] types
- [ ] messages
- [ ] `NaiveTime` timestamp helper test

### Integration testing
`test` module outside of `src`
- [ ] Test OUCH compliance (via emulated API?)

