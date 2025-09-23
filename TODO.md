
## TODO for crate:
- [ ] Test OUCH compliance (via emulator API?)
- [ ] Unit testing
- [ ] `README.md` minimal examples
- [ ] `async` and `log` features


## TODO for library:

### For client:
- [ ] Finish send/recv for `OuchClient`

## TODO for Messages
### For `OuchRequest`
- [ ] Comments & documentation

### For `OptionalAppendage`
- [ ] Options need types (e.g., `TagValue` and enums)
- [ ] Strong types for optional fields (there are many)

### For `OuchResponse`
- [x] `Timestamp` type?
- [x] `EventCode` type
- [ ] `CancelReason`
- [ ] `BrokenReason`
- [ ] `LiquidityFlag`
- [ ] `AiqStrategy`
- [ ] `OrderRefNum` (nasdq-assigned)
- [ ] `MatchNumber` Assigned by exchange to identify trade

- [ ] `parse` and `options` methods

- [x] `SystemEvent`
- [x] `OrderAccepted`
- [x] `OrderReplaced`
- [x] `OrderCanceled`
- [x] `AiqCanceled`
- [x] `OrderExecuted`
- [x] `BrokenTrade`
- [x] `Rejected`
- [x] `CancelPending`
- [x] `CancelReject`
- [x] `OrderPriorityUpdate`
- [x] `OrderModified`
- [x] `OrderRestated`
- [x] `MassCancelResponse`
- [x] `DisableOrderEntryResponse`
- [x] `EnableOrderEntryResponse`
- [x] `AccountQueryResponse`

