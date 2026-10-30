# Changelog

All notable changes to Evergreen are recorded here. The format follows
[Keep a Changelog 1.1.0](https://keepachangelog.com/en/1.1.0/), and the crates
follow [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

Each pull request that changes shipped code also adds a detailed note under
`docs/release-notes/`. This file is the short, per-version summary of those notes.

## [Unreleased]

## [0.1.0] - 2026-10-29

Phase 0: the workspace and the on-chain library. Not yet published to crates.io.

### Added

- `evergreen-core`, a `no_std` library for Soroban contracts:
  - `TtlPolicy`, a `const` rule for when and how far to extend an entry's TTL,
    rejecting a zero target or a threshold above the target.
  - `TtlPolicy::resolve`, which clamps the policy to the network's live maximum
    TTL read from the ledger.
  - `PersistentStore` and `InstanceStore`, typed storage handles that extend TTL
    on write and on read.
  - `Tier`, the three Soroban storage tiers as a named type.
  - `EvergreenError::InvalidPolicy`.
- Interface stubs for `evergreen-monitor`, `evergreen-migrate`, `evergreen-lint`
  and the `evergreen` CLI (`evergreen-cli`). The types and command surface are
  in place; the operations are not implemented yet.
- The `evergreen-js` TypeScript binding stub, mirroring the monitor's types.
- Example contracts: `token-with-policy`, `rarely-written-state` and
  `migration-demo`.
- The mdBook under `docs/` with the introduction, state archival background,
  architecture and getting started chapters.

<!-- No release tag exists yet, so the links point at the Phase 0 commit.
     Switch them to compare/v0.1.0...HEAD and releases/tag/v0.1.0 once the
     tag is pushed. -->
[Unreleased]: https://github.com/vegaforge/evergreen/compare/2b4867b...HEAD
[0.1.0]: https://github.com/vegaforge/evergreen/commit/2b4867b
