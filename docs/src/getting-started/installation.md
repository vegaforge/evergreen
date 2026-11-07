# Installation

`evergreen-core` is a Rust library for Soroban contracts. You add it to a
contract crate next to `soroban-sdk`.

## Prerequisites

- Rust through [rustup](https://rustup.rs). `evergreen-core` needs Rust 1.91 or
  newer.
- The Soroban Wasm target. Current Soroban builds contracts for `wasm32v1-none`,
  not the older `wasm32-unknown-unknown`:

  ```bash
  rustup target add wasm32v1-none
  ```

- The [Stellar CLI](https://developers.stellar.org/docs/tools/cli) if you want to
  deploy or read network settings. Building and testing do not need it.

## Add the dependency

`evergreen-core` is published on
[crates.io](https://crates.io/crates/evergreen-core). It is built against
`soroban-sdk` 26:

```toml
[dependencies]
soroban-sdk = "26"
evergreen-core = "0.1"

[dev-dependencies]
soroban-sdk = { version = "26", features = ["testutils"] }
```

The crate is `no_std`, like every Soroban contract, and adds no dependencies of
its own beyond `soroban-sdk`. Its API documentation is on
[docs.rs](https://docs.rs/evergreen-core).

## Check that it builds

```bash
cargo build --target wasm32v1-none --release
```

Next, follow the [quick start](quick-start.md) to keep your first entry alive.
