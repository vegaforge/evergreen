---
date: 2026-10-29
feature: crates.io-ready metadata for evergreen-core
scope: chore
scenario-impact: none
---

# Crate metadata for evergreen-core

`evergreen-core` now carries correct package metadata, so a first
`cargo publish` produces a crates.io page that points at the right places. No
code changed and nothing was published.

- `repository` is `https://github.com/vegaforge/evergreen` for every crate. It
  pointed at `Signor1/evergreen`, which does not exist. The mdBook's
  `git-repository-url` is fixed the same way.
- `rust-version` is `1.91`, the minimum `soroban-sdk 26.1.0` declares. It was
  `1.81`, which cannot build the workspace.
- `evergreen-core` adds `documentation` (docs.rs) and the `wasm` and
  `cryptography::cryptocurrencies` categories, ships the Apache-2.0 `LICENSE`
  text inside the package, and leaves the Soroban test snapshots out of it.
- `homepage` is deliberately unset until the mdBook has its own URL, because
  Cargo flags a `homepage` that repeats `repository`.
- The workspace comment on the release profile now gives the current mainnet
  contract size limit (131,072 bytes) with its date and source.

Closes #15.
