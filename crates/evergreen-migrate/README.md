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
`MigrationError::NotImplemented`. This is a Phase 4 deliverable, tracked in
[#12](https://github.com/vegaforge/evergreen/issues/12) and
[#13](https://github.com/vegaforge/evergreen/issues/13).

## Upgrade and migration sequencing

The storage transform does not run inside the Wasm upgrade call. Only the
contract can write its own storage, and new Wasm takes effect only after the
current invocation finishes, so the migration runs through a separate,
admin-guarded `migrate` entrypoint in the new version. This engine dry-runs the
transforms and sequences the two calls. The
[roadmap](../../docs/src/roadmap.md#does-the-storage-transform-share-the-upgrade-transaction)
explains the model.
