
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
- [ ] `encode` to byte string
- [ ] Comments & documentation
- [ ] Datatype for `UserRefNum`, `Price`, and others

### For `OptionalAppendage`
- [ ] Options need types (e.g., `TagValue` and enums)
- [ ] Strong types for optional fields (there are many)

### For `OuchResponse`
- [ ] `SystemEvent`
- [ ] `OrderAccepted`
- [ ] `OrderReplaced`
- [ ] `OrderCanceled`
- [ ] `AiqCanceled`
- [ ] `OrderExecuted`
- [ ] `BrokenTrade`
- [ ] `Rejected`
- [ ] `CancelPending`
- [ ] `CancelReject`
- [ ] `OrderPriorityUpdate`
- [ ] `OrderModified`
- [ ] `OrderRestated`
- [ ] `MassCancelResponse`
- [ ] `DisableOrderEntryResponse`
- [ ] `EnableOrderEntryResponse`
- [ ] `AccountQueryResponse`

