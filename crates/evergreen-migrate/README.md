# evergreen-migrate

Safe contract upgrade plus declarative storage migration, the deepest and most
valuable piece of Evergreen.

Soroban supports upgrading a contract's Wasm. Changing the shape of stored data
across that upgrade has no tooling. This crate provides declarative migrations: a
migration describes a transformation of stored data, such as renaming a key,
re-laying-out a value, or moving a field between tiers.

The dry-run is the safety property. The engine computes the full diff against
current ledger state before anything changes, the same discipline a database
migration tool provides. Every migration also records a rollback plan.

## Status

Stub. The types in `src/lib.rs` are the intended interface; the methods return
`MigrationError::NotImplemented`. This is a Phase 4 deliverable. See PRD section
6.3 and 7.

## Open question being carried

The safest sequencing of the storage transform relative to the Wasm upgrade
transaction, same transaction or a guarded pair, is PRD open question 15.
