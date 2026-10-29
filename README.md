<h1 align="center">Evergreen</h1>

<p align="center">
  A storage lifecycle, state-archival, and upgrade framework for Soroban.
</p>

<p align="center">
  <a href="#license"><img alt="License: Apache-2.0" src="https://img.shields.io/badge/license-Apache--2.0-blue.svg"></a>
  <img alt="Status: Phase 0" src="https://img.shields.io/badge/status-Phase%200-green.svg">
  <img alt="Language: Rust" src="https://img.shields.io/badge/language-Rust-orange.svg">
</p>

---

## What Evergreen is

Evergreen keeps a Soroban contract's storage alive so its authors do not have to think about archival. You declare a storage policy once, and TTL extension, tier discipline, archival recovery, and upgrade-time data migration follow from it.

Soroban fights state bloat with state archival. Every ledger entry has a time-to-live measured in ledgers. Persistent and instance entries are archived when their TTL lapses and must be explicitly restored before they can be used again. Temporary entries are deleted outright and can never come back. Choosing the right tier, extending TTLs before they lapse, handling archival, and migrating data across an upgrade are all manual today, expressed as low-level SDK calls and CLI commands. The official docs describe this as a learning curve.

Evergreen is the high-level abstraction that learning curve is missing. The goal is simple to state: set the policy once and stop thinking about archival.

## Why it matters

Every Soroban contract touches storage, so the cost of the manual model is paid by the whole developer population, repeatedly. The sharp edges are real:

- **Tier selection has no guardrail.** Put a balance in temporary storage and it can vanish for good. Put ephemeral data in persistent storage and you pay rent on it forever.
- **TTL extension is easy to forget.** A contract that does not extend its entries will watch them archive. A single CLI extension tops out near 30 days, so long-lived contracts need a recurring process that most teams do not build until something breaks.
- **Restoration is fiddly.** Building a restore footprint by hand is error prone, and the failure mode in tests is a host panic.
- **Upgrades have no migration story.** Upgrading the Wasm is supported. Changing the shape of stored data across that upgrade has no tooling and no safety net.
- **Cost is opaque.** There is no easy way to see what storage will cost over time or which entries are about to lapse.

None of this is a flaw in Soroban. It is the price of a ledger that refuses to grow without bound. Evergreen amortizes that price behind one policy.

## Architecture

Evergreen has three layers. They are deliberately separable, so a team can adopt one without the others.

```
 ON-CHAIN                OFF-CHAIN                 TOOLING
 evergreen-core          evergreen-monitor         evergreen-cli
 - typed storage         - watches entries         - inspect, forecast
   wrappers                over RPC                - extend, restore
 - declarative TTL       - predicts archival       evergreen-migrate
   policy                - extends early           - wasm upgrade
 - extend on access      - restores archived       - storage migration
 - tier as a type        - respects a budget         with dry-run

                         evergreen-lint            evergreen-js
                         - flags tier misuse       - TypeScript bindings
```

### Layer 1: the on-chain library (`evergreen-core`)

A contract author declares a key's tier and TTL policy once. Reads and writes go through a typed handle that extends the entry's TTL on access, within the declared thresholds. There is no hand-written extension logic in the contract.

The policy is plain data: when remaining TTL drops below a threshold, extend the entry to a target. The handle turns that into the right host call at the right moment, and clamps every extension to the network's live maximum TTL, which it reads from the ledger rather than hardcoding. Extending a healthy entry is a no-op at the host level, so applying the policy on every access is cheap.

This is the only layer that has to be Rust: Soroban contracts are `no_std` Rust on `soroban-sdk`. It is also the foundation every other layer builds on, which is why it is the piece that is fully implemented first.

### Layer 2: the off-chain monitor (`evergreen-monitor`)

Some entries are written so rarely that they would lapse before the next natural write extends them. The on-chain library cannot help an entry that nothing is accessing. The monitor closes that gap. It watches a deployed contract over Stellar RPC, computes each entry's time to archival, and extends entries that are approaching the threshold or restores entries that have already archived. It works within a configurable rent budget and raises an alert when an entry is at risk it cannot cover.

### Layer 3: the tooling (`evergreen-cli`, `evergreen-migrate`, `evergreen-lint`)

The CLI exposes inspection (every entry with its TTL, time to archival, and projected rent cost), extension and restoration wrappers, and the upgrade and migration toolkit.

Migration is the hardest and most valuable piece. A migration describes a transformation of stored data: rename a key, re-lay-out a value, move a field between tiers. The engine dry-runs it against current ledger state to show the full diff before anything changes, executes it alongside the Wasm upgrade, and records a rollback plan. The dry-run is the safety gate, the same discipline a database migration tool provides.

The lint flags the most dangerous storage mistake, durable data routed to temporary storage, turning a silent bug into a visible, reviewable choice.

## Repository layout

```
evergreen/
  crates/
    evergreen-core/       on-chain Rust library (soroban-sdk)
    evergreen-monitor/    off-chain monitor
    evergreen-cli/        command line tool
    evergreen-migrate/    upgrade and storage migration engine
    evergreen-lint/       tier-misuse lints
  bindings/
    evergreen-js/         thin TypeScript bindings
  examples/
    token-with-policy/    a token that keeps balances alive with no TTL code
    rarely-written-state/ shows the gap the monitor fills
    migration-demo/       a v1 contract for the migration story
  docs/                   mdBook documentation
```

## Status

Evergreen is being built in phases. This repository is at **Phase 0**.

| Component | Status |
| --------- | ------ |
| `evergreen-core` | Implemented: typed persistent and instance wrappers, declarative TTL policy, extend on access, network-bounded extension |
| `token-with-policy` | Implemented: keeps balances alive with no hand-written extension logic |
| `evergreen-monitor` | Interface defined, implementation pending (Phase 2) |
| `evergreen-cli` | Command surface defined, implementation pending (Phase 3) |
| `evergreen-migrate` | Interface defined, implementation pending (Phase 4) |
| `evergreen-lint` | Interface defined, implementation pending (Phase 1) |
| `evergreen-js` | Binding surface stubbed |

## Quickstart

Route a balance through `evergreen-core` and it stays alive on its own:

```rust
use evergreen_core::{PersistentStore, TtlPolicy};
use soroban_sdk::{Address, Env};

// Extend when under ~1 day of TTL, push back to ~30 days. Checked at compile time.
const BALANCE_POLICY: TtlPolicy = match TtlPolicy::new(17_280, 518_400) {
    Ok(policy) => policy,
    Err(_) => panic!("balance policy constants are invalid"),
};

fn credit(env: &Env, who: &Address, amount: i128) {
    let balances = PersistentStore::new(env, BALANCE_POLICY);
    let current: i128 = balances.get(who).unwrap_or(0);
    balances.set(who, &(current + amount));
    // No extend_ttl call: the store kept the entry alive.
}
```

The `examples/token-with-policy` contract is a complete working version.

## Building

Evergreen is a Cargo workspace. Contracts build to the `wasm32v1-none` target, which current Soroban requires (the older `wasm32-unknown-unknown` target is rejected by `soroban-sdk` on recent Rust).

```bash
# From the repository root.
rustup target add wasm32v1-none

cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo build -p token-with-policy --target wasm32v1-none --release
```

## Contributing

Each crate is an independently scoped contributor target. See [CONTRIBUTING.md](CONTRIBUTING.md) for setup and conventions.

## Security

Evergreen never holds funds and never takes custody. It only extends, restores, and migrates a contract's own storage. To report a vulnerability, see [SECURITY.md](SECURITY.md).

## License

Apache-2.0. See [LICENSE](LICENSE).
