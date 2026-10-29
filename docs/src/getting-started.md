# Getting started

This page gets you building with `evergreen-core`, the on-chain library. It is
the only layer that is fully implemented today.

## Prerequisites

- A stable Rust toolchain.
- The Soroban Wasm target. Current Soroban builds contracts for `wasm32v1-none`,
  not the older `wasm32-unknown-unknown`:

  ```bash
  rustup target add wasm32v1-none
  ```

## Add the dependency

Inside a Soroban contract crate, depend on `evergreen-core` alongside
`soroban-sdk`:

```toml
[dependencies]
soroban-sdk = "26"
evergreen-core = { path = "../evergreen/crates/evergreen-core" }
```

## Declare a policy

A policy says when to extend an entry and how far. Declare it as a constant so it
is checked at compile time:

```rust
use evergreen_core::TtlPolicy;

// Extend once remaining TTL is at or below ~1 day, push it back to ~30 days.
// Values are in ledgers, about five seconds each.
const BALANCE_POLICY: TtlPolicy = match TtlPolicy::new(17_280, 518_400) {
    Ok(policy) => policy,
    Err(_) => panic!("balance policy constants are invalid"),
};
```

If the numbers cannot produce a valid extension, the build fails rather than
shipping a broken policy.

## Route storage through a handle

Use `PersistentStore` for data that must survive, and `InstanceStore` for small
global state. Both apply the policy on every read and write:

```rust
use evergreen_core::PersistentStore;
use soroban_sdk::{Address, Env};

fn credit(env: &Env, who: &Address, amount: i128) {
    let balances = PersistentStore::new(env, BALANCE_POLICY);
    let current: i128 = balances.get(who).unwrap_or(0);
    balances.set(who, &(current + amount));
    // No extend_ttl call: the store kept the entry alive.
}
```

## See it end to end

The `examples/token-with-policy` contract is a complete token that keeps its
balances alive with no hand-written extension logic. Its tests show the entry
staying alive across a long stretch of ledgers, extended only by ordinary reads.

```bash
cargo test -p token-with-policy
cargo build -p token-with-policy --target wasm32v1-none --release
```

## What this does not cover

`evergreen-core` keeps frequently accessed entries alive. An entry that is
written once and then left untouched can still lapse, because there is no access
to extend it. That is the job of the off-chain monitor, which is on the roadmap.
The `examples/rarely-written-state` contract shows exactly this case.
