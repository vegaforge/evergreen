# Architecture

Evergreen has three layers. They are deliberately separable, so a team can adopt
one without the others.

## Layer 1: the on-chain library

`evergreen-core` is a Rust library on top of `soroban-sdk`. It gives a contract
author typed storage handles with a TTL policy attached.

A policy is plain data: a tier, a threshold, and a target. The author declares it
once, and reads and writes go through a handle that extends the entry's TTL on
access, within the declared thresholds. There is no hand-written extension logic
in the contract.

Two design points make this safe:

- Every extension is clamped to the network's live maximum TTL, read from the
  ledger, so Evergreen never submits an extension the host would reject.
- The tier is part of the type. Choosing a tier is an explicit call to
  `PersistentStore` or `InstanceStore`, not an unlabelled storage call, which
  makes the choice visible in review.

This layer is the foundation. It is also the only layer that has to be Rust,
because Soroban contracts are `no_std` Rust on `soroban-sdk`.

## Layer 2: the off-chain monitor

Some entries are written so rarely that they would lapse before the next natural
write extends them. The on-chain library cannot help an entry that nothing is
accessing.

`evergreen-monitor` closes that gap. It watches a deployed contract over Stellar
RPC, computes each entry's time to archival from its TTL against the current
ledger, and extends entries approaching the threshold or restores entries that
have already archived. It works within a configurable rent budget, and raises an
alert when an entry is at risk it cannot cover. The monitor is the safety net for
the write-rarely, must-survive case.

## Layer 3: the tooling

`evergreen-cli` is the command surface for everything outside a long-running
monitor: inspect entries with their TTL, time to archival, and projected cost;
extend and restore; and drive upgrades and migrations.

`evergreen-migrate` is the deepest piece. A migration describes a transformation
of stored data, such as renaming a key, re-laying-out a value, or moving a field
between tiers. The engine dry-runs it against current ledger state to show the
full diff before anything changes, executes it alongside the Wasm upgrade, and
records a rollback plan. The dry-run is the safety gate.

`evergreen-lint` flags the most dangerous storage mistake, durable data routed to
temporary storage, turning a silent bug into a visible choice.

`evergreen-js` is a thin TypeScript binding so teams whose operational tooling is
Node-based can drive the monitor and CLI without adopting Rust on the ops side.
It is a convenience surface, not a second implementation.

## Why Rust

The on-chain library has no alternative: Soroban contracts are `no_std` Rust.
Extending that choice to the monitor and CLI is deliberate. The Stellar CLI is
Rust, and the official `stellar-xdr` and Soroban crates are Rust, so reusing them
avoids re-implementing XDR handling, footprint construction, and ledger-entry
parsing in another language. A single Rust toolchain also ships as a static
binary, which is the right distribution for a CLI, and keeps the whole framework
in one language.
