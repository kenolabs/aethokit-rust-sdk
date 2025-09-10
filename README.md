# Aethokit Rust SDK (Rust)

Rust SDK for the **Aethokit** gas sponsorship API — a mirror of the TypeScript SDK.

- ✅ Minimal, ergonomic API
- ✅ Async HTTP using `reqwest`
- ✅ Strongly-typed requests/responses via `serde`


## Install

Add to your `Cargo.toml`:

```toml
[dependencies]
aethokit = "0.1"
tokio = { version = "1", features = ["macros", "rt-multi-thread"] } # for async runtime in binaries/tests
```

## Usage

```rust
use aethokit::{Aethokit, AethokitConfig, SponsorTxRequest};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  // load your GAS KEY from environment
  let gas_key = std::env::var("AETHOKIT_GAS_KEY")
    .expect("set AETHOKIT_GAS_KEY in your environment");

  // initialize aethokit config with network
  let aethokit_client = Aethokit::new(AethokitConfig {
    gas_key,
    rpc_or_network: Some("mainnet".into()), // or None if you want default (devnet)
  })?;

  // initialize aethokit config with rpc_url
  let aethokit_client = Aethokit::new(AethokitConfig {
    gas_key,
    rpc_or_network: Some("private-rpc-url".into()), // or None if you want default
  })?;

  // fetch gas address
  let addr = aethokit_client.get_gas_address().await?;
  println!("Gas address: {}", addr);

  // sponsor the tx and get hash
  let hash = aethokit_client.sponsor_tx("<SERIALIZED_TX>".into()).await?;
  println!("Hash: {}", hash);

  Ok(())
}
```

For usage please refer to the examples [here](https://github.com/kenolabs/aethokit-rust-sdk/tree/main/examples).
