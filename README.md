
# slouch
Simple [OUCH 5.0](https://www.nasdaqtrader.com/content/technicalsupport/specifications/TradingProducts/OUCH5.0.pdf) client for communication with NASDAQ.<br>
Contains strong types for messages and a simple client to send and receive them.

#### WARNING: Work-in-progress
`slouch` should be presumed non-functional until integration testing has been completed.
If you are willing and able to assist with integration testing, please leave a response under 
[this issue](https://github.com/j-stach/slouch/issues/1).

## Use
1. Add `slouch` to your Rust project (v2024 or more recent), with `cargo add slouch`.
2. Create an `OuchClient` to handle messages.
When the client is created, it will attempt to query the pre-existing OUCH account registered for the host IP address. 
Client setup will fail if an `AccountQueryResponse` is not received from the server.
```rust
use slouch::OuchClient;

use std::net::SocketAddr;
use std::time::Duration;

let mut client = OuchClient::new(
    SocketAddr::new(/* IP address */, /* Port number */), // OUCH Server addr
    Duration::from_secs(5)
).unwrap();
```
3. Send a message and receive a response. 
Response is an enum that can be matched to extract message values.
```rust
use slouch::account_query;

client.send(account_query!()).unwrap();

let response = client.recv().unwrap();

use slouch::msg::OuchResponse::*;
match response {
    AccountQueryResponse(val) => {/* Do something with val */},
    _ => {/* Do something else */}
}
```
4. Requests all have macros that simplify message creation.
Check the documentation comment for each macro to see how it is used. <br>
As an example:
```
use slouch::{ cancel, types::UserRefNum };

client.send(cancel!{
    user_ref_num: UserRefNum::new(),
    quantity: 0u32,
}).unwrap();
```
    

## Development
`slouch` is a work-in-progress. <br>
Development history and current tasks are tracked in [TODO.md](TODO.md).

Developer resources:
- [OUCH 5.0 specification](https://www.nasdaqtrader.com/content/technicalsupport/specifications/TradingProducts/OUCH5.0.pdf)
- [AIQ Strategy Values](https://www.nasdaqtrader.com/TraderNews.aspx?id=ETU2023-8)

