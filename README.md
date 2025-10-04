
# slouch
Simple [OUCH 5.0](https://www.nasdaqtrader.com/content/technicalsupport/specifications/TradingProducts/OUCH5.0.pdf) client for communication with NASDAQ.<br>
Contains strong types for messages and a simple client to send and receive them.

### WARNING: Work-in-progress
`slouch` should be presumed experimental and non-functional until integration testing has been completed.
If you are willing and able to assist with integration testing, please leave a response under 
[this issue](https://github.com/j-stach/slouch/issues/1).

## Use
1. Add `slouch` to your Rust project (`v2024` or more recent):
```
cargo add slouch
```
2. Create an `OuchClient` to handle messages.
When the client is created, it will attempt to query the pre-existing OUCH account registered for the host IP address. 
Client setup will fail if an `AccountQueryResponse` is not received from the server.
```rust
use slouch::OuchClient;

use std::net::SocketAddr;
use std::time::Duration;

let mut client = OuchClient::new(
    SocketAddr::new(/* IP address */, /* Port number */), // OUCH Server port
    Duration::from_secs(5) // Timeout duration
).unwrap();
```
3. Send a message and receive a response. <br>
`OuchResponse` is an enum that can be matched to extract message values.
```rust
use slouch::account_query;

let request = account_query!();
client.send(request).unwrap();

let response = client.recv().unwrap();

use slouch::msg::OuchResponse::*;
match response {
    AccountQueryResponse(aqr) => {
        let _time: chrono::NaiveTime = aqr.timestamp();
        let _num: UserRefNum = aqr.next_user_ref_num();
        let _opts: &Vec<TagValue> = aqr.options();
    },
    _ => {/* Do something else */}
}
```
4. Each `OuchRequest` has a macro to simplify message creation. <br>
Check the documentation comment for a macro to see how it is used.
```rust
use slouch::{ enter, types::* };

let request = enter!{
    user_ref_num: client.new_user_ref_num(),
    side: Side::Buy,
    quantity: 69u32,
    symbol: StockSymbol::new("STONKS").unwrap(),
    price: Price::new(3, 5000).unwrap(),
    time_in_force: TimeInForce::Day,
    display: Display::Visible,
    capacity: Capacity::Agency,
    intermarket_sweep_eligibility: false,
    cross_type: CrossType::Opening,
    order_token: OrderToken::new("OrderToken").unwrap()
};

client.send(request).unwrap();
```
5. Client logging is provided by the [`log`](https://docs.rs/log/latest/log/) crate and can be enabled through the `logs` feature. 
An asynchronous version of the client is supported by the [`tokio`](https://docs.rs/tokio/latest/tokio/) crate and can be enabled through the `async` feature.
By default, `OuchClient` is synchronous and its events are not logged.
```toml
# Cargo.toml
[dependencies]
slouch = { version = "0.0.0", features = ["logs", "async"] }
```
6. `OuchRequest` and `OuchResponse` can also be used without an `OuchClient`.
```rust
use std::net;
use slouch::{ account_query, msg::OuchResponse };

// OUCH Server port
let addr = net::SocketAddr::new(/* IP address */, /* Port number */); 
let mut stream = net::TcpStream::connect(addr).unwrap();

let bytes = account_query!().to_bytes();
stream.write_all(&bytes).unwrap();

let mut buffer = vec![];
let n = stream.read(&mut buffer).unwrap();
let response = OuchResponse::try_from(&self.buffer[..n]).unwrap();
```
    

## Development
Development history and current tasks are tracked in [TODO.md](TODO.md).

Developer resources:
- [OUCH 5.0 Specification](https://www.nasdaqtrader.com/content/technicalsupport/specifications/TradingProducts/OUCH5.0.pdf)
- [AIQ Strategy Values](https://www.nasdaqtrader.com/TraderNews.aspx?id=ETU2023-8)

Contributions are welcome. Submit issues and pull requests to this repository.
