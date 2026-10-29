# token-with-policy

A small token contract that stores balances through `evergreen-core`.

The point of the example: there is no `extend_ttl` call anywhere in the contract.
Balances go through a `PersistentStore` and the admin through an `InstanceStore`,
both carrying a TTL policy, so every read and write keeps the data alive on its
own.

This is the Phase 0 deliverable: an example contract that keeps its storage alive
with no hand-written extension logic.

## What the tests show

- Minting and transferring work as expected, with authorization enforced.
- A balance lands at the policy target after a write.
- After the ledger advances to the edge of archival, a plain balance read brings
  the entry back to the target. The contract never calls `extend_ttl`; the store
  does it.

```bash
cargo test -p token-with-policy
cargo build -p token-with-policy --target wasm32v1-none --release
```
