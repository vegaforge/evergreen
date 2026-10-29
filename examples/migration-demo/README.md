# migration-demo

Version 1 of a contract, used to demonstrate a storage migration across an
upgrade.

This crate is the "before" state. It stores a profile under one key shape and
reports its storage version as 1. The companion `evergreen-migrate` engine will
define a migration to version 2 that reshapes this data, dry-run it to show the
full diff, and execute it across a Wasm upgrade with a recorded rollback plan.

The migration itself lands with `evergreen-migrate` in Phase 4. See PRD section
6.3. For now this contract gives that work a concrete v1 to migrate from.

```bash
cargo test -p migration-demo
cargo build -p migration-demo --target wasm32v1-none --release
```
