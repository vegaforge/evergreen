# migration-demo

Version 1 of a contract, kept as the "before" state for a storage migration
across a Wasm upgrade.

Source: [`examples/migration-demo`](https://github.com/vegaforge/evergreen/tree/main/examples/migration-demo)

## What it shows

The contract stores a profile under one key shape and reports its storage
version as 1. It gives the migration work something concrete to migrate from.

## What comes next

The migration itself is not built yet. Two issues add it:

- [#12](https://github.com/vegaforge/evergreen/issues/12) makes this contract
  upgradeable and adds a version 2 with an admin-guarded, version-monotonic,
  idempotent `migrate` entrypoint.
- [#13](https://github.com/vegaforge/evergreen/issues/13) adds the dry-run
  planner and rollback plan in `evergreen-migrate`.

Why the migration runs in its own call rather than inside the upgrade is
explained in the
[roadmap's design questions](../project/roadmap.md#does-the-storage-transform-share-the-upgrade-transaction).

## Run it

```bash
cargo test -p migration-demo
cargo build -p migration-demo --target wasm32v1-none --release
```
