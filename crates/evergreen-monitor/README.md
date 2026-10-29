# evergreen-monitor

The off-chain safety net. It watches a deployed contract over Stellar RPC,
predicts which entries are approaching archival, and extends or restores them
before they lapse, within a rent budget.

The on-chain library keeps frequently accessed entries alive on its own. This
crate handles the entries that are written too rarely for that to work: it
extends them on a schedule between accesses, and restores them if they have
already archived.

## Status

Stub. The types in `src/lib.rs` are the intended interface; the methods return
`MonitorError::NotImplemented`. This is a Phase 2 deliverable. See PRD section
6.2.

## Planned dependencies

`stellar-rpc-client` for reading entries and submitting transactions, and
`stellar-xdr` for ledger keys and footprints.
