# rarely-written-state

A contract whose one piece of state is written once and then mostly left alone.
It exists to show the limit of the on-chain library.

Source: [`examples/rarely-written-state`](https://github.com/vegaforge/evergreen/tree/main/examples/rarely-written-state)

## What it shows

`evergreen-core` extends an entry's TTL when the entry is accessed. A contract
that is called often keeps itself alive. This one is not: after the first write
it might sit untouched for longer than its TTL, and extend-on-access cannot help
an entry that nothing touches. The entry lapses and is archived.

That gap is what the off-chain `evergreen-monitor` is for. It will watch the
entry over RPC and extend it on a schedule between accesses. The work is tracked
in [Phase 2 of the roadmap](../project/roadmap.md#phase-2-the-off-chain-monitor).

## Run it

```bash
cargo test -p rarely-written-state
cargo build -p rarely-written-state --target wasm32v1-none --release
```
