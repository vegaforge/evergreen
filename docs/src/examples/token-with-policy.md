# token-with-policy

A small token contract whose storage stays alive with no `extend_ttl` call
anywhere in its code.

Source: [`examples/token-with-policy`](https://github.com/vegaforge/evergreen/tree/main/examples/token-with-policy)

## What it shows

- Balances go through a `PersistentStore` and the admin through an
  `InstanceStore`, each with its own policy constant (threshold about 1 day,
  target about 30 days).
- Minting and transferring work as expected, with authorization enforced.
- A balance lands at the policy target after a write.
- After the ledger advances to the edge of archival, a plain balance read brings
  the entry back to the target.

## Run it

From a clone of the repository:

```bash
cargo test -p token-with-policy
cargo build -p token-with-policy --target wasm32v1-none --release
```

The test that proves the entry stays alive is walked through in
[Testing storage lifetimes](../guides/testing.md).
